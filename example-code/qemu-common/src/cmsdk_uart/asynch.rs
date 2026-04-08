//! # Asynchronous UART support
//!
//! Currently, this module only provides asynchronous TX support and not RX support.
//!
//! The module name is `asynch` and not `async` because `async` is a keyword and `asynch` is not.

use core::{
    convert::Infallible,
    sync::atomic::{
        AtomicPtr, AtomicUsize,
        Ordering::{Acquire, Relaxed, Release},
    },
};

use atomic_waker::AtomicWaker;

use crate::cmsdk_uart::basic;

/// Currently, a maximum of 5 CMSDK UART instances are supported.
pub const MAX_WAKERS: usize = 5;

/// Hold the state for our UARTs
static UART_STATE: [UartState; MAX_WAKERS] = [const { UartState::new() }; MAX_WAKERS];

/// Hold the async state for one UART
struct UartState {
    /// UART base pointer
    uart_base: AtomicUsize,
    /// Used to notify the executor when a transfer is done
    waker: AtomicWaker,
    /// The buffer currently being transferred (or null if no transfer in progress)
    ///
    /// This also acts as the in-progress flag (we are in-progress when this is non-null)
    tx_buffer: AtomicPtr<u8>,
    /// The length of the buffer being transferred, in bytes
    tx_length: AtomicUsize,
    /// The numbers of bytes transferred so far
    tx_transmitted: AtomicUsize,
}

impl UartState {
    /// Create a new, empty, UartState
    const fn new() -> UartState {
        UartState {
            waker: AtomicWaker::new(),
            tx_buffer: AtomicPtr::new(core::ptr::null_mut()),
            tx_length: AtomicUsize::new(0),
            tx_transmitted: AtomicUsize::new(0),
            uart_base: AtomicUsize::new(0),
        }
    }
}

/// Waker limit exceeded. This module only supports a maximum of [MAX_WAKERS] wakers.
#[derive(Debug)]
pub struct WakerLimitExceededError;

/// Holds the information we need to handle a UART TX interrupt.
pub struct InterruptCtx {
    uart_state: &'static UartState,
}

impl InterruptCtx {
    /// Handle the UART TX Interrupt
    ///
    /// # Safety
    ///
    /// This function must only be called from the UART TX interrupt context.
    pub unsafe fn handle_irq(&mut self) {
        let uart_state = self.uart_state;
        defmt::debug!(
            "on_interrupt_tx(state @ 0x{=usize:08x})",
            uart_state as *const UartState as usize
        );
        // Safety: We are called in a UART interrupt, so we're safe to talk to the TX side of the UART
        let base = uart_state.uart_base.load(Relaxed);
        if base == 0 {
            panic!("TX fired on invalid UART?!");
        }
        let mut tx = unsafe { crate::cmsdk_uart::Tx::steal(base) };
        tx.clear_interrupts();
        if !tx.regs().read_control().txie() || tx.regs().read_status().txf() {
            // TX Interrupt is not enabled, or TX FIFO is Full - we cannot proceed
            defmt::warn!("Spurious on_interrupt_tx() call!");
            return;
        }
        let tx_buffer = uart_state.tx_buffer.load(Relaxed);
        let tx_length = uart_state.tx_length.load(Relaxed);
        let tx_transmitted = uart_state.tx_transmitted.fetch_add(1, Relaxed);
        // No transfer active.
        if tx_buffer.is_null() {
            return;
        }
        if tx_transmitted >= tx_length || tx_length == 0 {
            defmt::debug!("TX Done! Waking...");
            // Transfer is done. Notify executor and set completion flag.
            uart_state.tx_buffer.store(core::ptr::null_mut(), Release);
            uart_state.waker.wake();
            return;
        }

        let byte = unsafe { tx_buffer.add(tx_transmitted).read() };
        defmt::debug!("TX 0x{:02x}", byte);

        // Write next byte of transfer. We do not expect this to block.
        tx.write(byte).expect("TX IRQ should be non-blocking");
    }
}

/// Asynchronous UART Transmit driver
///
/// Like [`cmsdk_uart::basic::Tx`](crate::cmsdk_uart::basic::Tx), but async.
pub struct AsyncTx {
    basic_tx: basic::Tx,
    uart_state: &'static UartState,
}

impl AsyncTx {
    /// Create a new asynchronous TX driver from a blocking one.
    pub fn new(mut basic_tx: basic::Tx) -> Result<(Self, InterruptCtx), WakerLimitExceededError> {
        /// TX index which is incremented every time as asynchronous TX driver is created.
        ///
        /// This ensures we don't let the user make more drivers than we have space in UART_STATE for.
        static NEXT_STATE_IDX: AtomicUsize = AtomicUsize::new(0);

        let current_index = NEXT_STATE_IDX.fetch_add(1, Relaxed);

        let Some(uart_state) = UART_STATE.get(current_index) else {
            return Err(WakerLimitExceededError);
        };

        // Stash the UART base address for use later
        uart_state
            .uart_base
            .store(unsafe { basic_tx.regs().ptr() as usize }, Relaxed);
        // set up our UART:

        // just in case anything is pending
        basic_tx.clear_interrupts();

        // we want the TX interrupt to fire when the FIFO is empty
        basic_tx.enable_interrupts();

        // Ensure the UART is enabled (in case they disabled it before)
        basic_tx.enable();

        Ok((
            AsyncTx {
                basic_tx,
                uart_state: &UART_STATE[current_index],
            },
            InterruptCtx {
                uart_state: &UART_STATE[current_index],
            },
        ))
    }

    /// Asynchronously write data to the UART.
    ///
    /// Completes when all bytes have sent from the TX FIFO.
    pub async fn write(&mut self, buf: &[u8]) {
        defmt::debug!("Transmitting {=[u8]:02x}", buf);
        // note: we must not try and transmit an empty buffer - bail out early instead
        if buf.is_empty() {
            return;
        }
        Transmission::new(self, buf).await;
    }
}

impl embedded_io_async::ErrorType for AsyncTx {
    type Error = Infallible;
}

impl embedded_io_async::Write for AsyncTx {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.write(buf).await;
        Ok(buf.len())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        while self.basic_tx.regs().read_status().txf() {
            core::hint::spin_loop();
        }
        Ok(())
    }
}

/// Represents an ongoing asynchronous UART transmission, which can be polled.
///
/// The lifetime annotation `'tx` represents the lifetime of the Async UART the transmission is borrowing.
pub struct Transmission<'uart> {
    tx: &'uart mut AsyncTx,
}

impl<'uart> Transmission<'uart> {
    /// Create a new asynchronous future for a write/transmit operation.
    ///
    /// Will send the given buffer under interrupt, producing
    /// `core::task::Poll::Ready` once complete.
    ///
    /// Do not pass a zero-length slice - this will panic.
    ///
    /// We can only keep this object whilst *both* the Async TX UART *and* the buffer are alive.
    fn new(tx_async: &'uart mut AsyncTx, data: &'uart [u8]) -> Self {
        defmt::debug!(
            "Creating Transmission(data=0x{=usize:08x}, len={})",
            data.as_ptr() as usize,
            data.len(),
        );
        assert!(!data.is_empty());
        // tx.inner.disable_interrupts();
        tx_async.uart_state.tx_length.store(data.len(), Relaxed);
        tx_async.uart_state.tx_transmitted.store(1, Relaxed);
        tx_async
            .uart_state
            .tx_buffer
            .store(data.as_ptr() as *mut u8, Release);
        // It is actually important to write AFTER the UART was enabled.
        defmt::debug!("TX 0x{:02x}", data[0]);
        tx_async.basic_tx.write(data[0]).unwrap();

        Self { tx: tx_async }
    }
}

impl core::future::Future for Transmission<'_> {
    type Output = ();

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        defmt::debug!("Polling Transmission complete...");
        self.tx.uart_state.waker.register(cx.waker());
        if self.tx.uart_state.tx_buffer.load(Acquire).is_null() {
            defmt::debug!("Ready!");
            core::task::Poll::Ready(())
        } else {
            defmt::debug!("Pending...");
            core::task::Poll::Pending
        }
    }
}

impl Drop for Transmission<'_> {
    fn drop(&mut self) {
        if !self.tx.uart_state.tx_buffer.load(Acquire).is_null() {
            self.tx.basic_tx.clear_interrupts();
            self.tx.basic_tx.disable_interrupts();
        }
    }
}
