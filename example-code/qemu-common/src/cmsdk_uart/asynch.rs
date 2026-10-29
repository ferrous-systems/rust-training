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
pub struct InterruptContext {
    uart_state: &'static UartState,
}

impl InterruptContext {
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
        if !tx.interrupt_enabled() || tx.tx_fifo_full() {
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
    pub fn new(
        mut basic_tx: basic::Tx,
    ) -> Result<(Self, InterruptContext), WakerLimitExceededError> {
        /// TX index which is incremented every time as asynchronous TX driver is created.
        ///
        /// This ensures we don't let the user make more drivers than we have space in UART_STATE for.
        static NEXT_STATE_IDX: AtomicUsize = AtomicUsize::new(0);

        let current_index = NEXT_STATE_IDX.fetch_add(1, Relaxed);

        let Some(uart_state) = UART_STATE.get(current_index) else {
            return Err(WakerLimitExceededError);
        };

        // Stash the UART base address for use later
        uart_state.uart_base.store(basic_tx.base_address(), Relaxed);
        // set up our UART:

        // just in case anything is pending
        basic_tx.clear_interrupts();

        // we want the TX interrupt to fire when the FIFO is empty
        basic_tx.enable_interrupt(true);

        // Ensure the UART is enabled (in case they disabled it before)
        basic_tx.enable(true);

        Ok((
            AsyncTx {
                basic_tx,
                uart_state: &UART_STATE[current_index],
            },
            InterruptContext {
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
        defmt::debug!(
            "Creating Transmission(data=0x{=usize:08x}, len={})",
            buf.as_ptr() as usize,
            buf.len(),
        );
        assert!(!buf.is_empty());
        self.uart_state.tx_length.store(buf.len(), Relaxed);
        self.uart_state.tx_transmitted.store(1, Relaxed);
        self.uart_state
            .tx_buffer
            .store(buf.as_ptr() as *mut u8, Release);
        // It is actually important to write AFTER the UART was enabled.
        defmt::debug!("TX 0x{:02x}", buf[0]);
        self.basic_tx.write(buf[0]).unwrap();

        let guard = TransferGuard(self);
        core::future::poll_fn(|cx| {
            defmt::debug!("Polling Transmission complete...");
            guard.0.uart_state.waker.register(cx.waker());
            if guard.0.uart_state.tx_buffer.load(Acquire).is_null() {
                defmt::debug!("Ready!");
                core::task::Poll::Ready(())
            } else {
                defmt::debug!("Pending...");
                core::task::Poll::Pending
            }
        })
        .await;
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
        while self.basic_tx.tx_fifo_full() {
            core::hint::spin_loop();
        }
        Ok(())
    }
}

/// This wrapper structure also acts as a cancellation guard.
///
/// It clears and disables interrupts on drop if the transfer has not finished regularly.
struct TransferGuard<'tx>(&'tx mut AsyncTx);

impl Drop for TransferGuard<'_> {
    fn drop(&mut self) {
        if !self.0.uart_state.tx_buffer.load(Acquire).is_null() {
            self.0.basic_tx.clear_interrupts();
            self.0.basic_tx.enable_interrupt(false);
        }
    }
}
