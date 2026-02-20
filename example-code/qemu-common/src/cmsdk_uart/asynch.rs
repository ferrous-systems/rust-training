//! # Asynchronous UART support
//!
//! Currently, this module provides asynchronous TX support.
use core::{
    cell::Cell,
    convert::Infallible,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use atomic_waker::AtomicWaker;
use critical_section::Mutex;

use crate::cmsdk_uart::Tx;

/// Currently, a maximum of 5 CMSDK UART instances are supported.
pub const MAX_WAKERS: usize = 5;

// Each CMSDK UART has a waker which is used to notify the executor when a transfer is done.
static TX_WAKERS: [AtomicWaker; 5] = [const { AtomicWaker::new() }; 5];
// Each CMSDK UART has a atomic completion flag which is used to check for transfer completion
// without a lock.
static TX_DONE: [AtomicBool; 5] = [const { AtomicBool::new(false) }; 5];

// The transfer information and context for each asynchronous CMSDK UART driver is tracked in
// this structure.
static TX_CONTEXTS: [Mutex<Cell<TranferContext>>; 5] =
    [const { Mutex::new(Cell::new(TranferContext::new())) }; 5];

/// TX index which is incremented every time as asynchronous TX driver is created.
static TX_INDEX: AtomicUsize = AtomicUsize::new(0);

/// Waker limit exceeded. This module only supports a maximum of [MAX_WAKERS] wakers.
#[derive(Debug)]
pub struct WakerLimitExceededError;

/// TX handler ID which is created when creating an asynchronous TX driver.
#[derive(Debug, Clone, Copy)]
pub struct TxHandlerId(usize);

/// This function should be called on CMSDK UART interrupts to allow asynchronous TX operations
/// to work.
pub fn on_interrupt_tx(tx: &mut Tx, id: TxHandlerId) {
    let raw_id = id.0;
    tx.clear_interrupts();
    if !tx.regs().read_control().txie() || tx.regs().read_status().txf() {
        return;
    }
    let mut context = critical_section::with(|cs| {
        let context_ref = TX_CONTEXTS[raw_id].borrow(cs);
        context_ref.get()
    });
    // No transfer active.
    if context.data_ptr.is_null() {
        return;
    }
    let slice = unsafe { core::slice::from_raw_parts(context.data_ptr, context.transfer_len) };
    let slice_len = slice.len();
    if context.progress >= slice_len || slice_len == 0 {
        // Transfer is done. Notify executor and set completion flag.
        TX_DONE[raw_id].store(true, core::sync::atomic::Ordering::Relaxed);
        TX_WAKERS[raw_id].wake();
        tx.disable_interrupts();
        return;
    }

    // Write next byte of transfer.
    tx.write(slice[context.progress]).unwrap();
    context.progress += 1;

    // Write back updated context structure.
    critical_section::with(|cs| {
        let context_ref = TX_CONTEXTS[raw_id].borrow(cs);
        context_ref.set(context);
    });
}

#[derive(Clone, Copy)]
struct TranferContext {
    // Data pointer of the data to be transmitted.
    data_ptr: *const u8,
    // Full transfer length.
    transfer_len: usize,
    // Current progress.
    progress: usize,
}

#[allow(clippy::new_without_default)]
impl TranferContext {
    pub const fn new() -> Self {
        Self {
            progress: 0,
            data_ptr: core::ptr::null(),
            transfer_len: 0,
        }
    }
}

// Safety: We only use this type wrapped in a mutex.
unsafe impl Sync for TranferContext {}
unsafe impl Send for TranferContext {}

/// Asycnhronous TX driver.
pub struct TxAsynch {
    inner: Tx,
    id: TxHandlerId,
}

impl TxAsynch {
    /// Create a new asynchronous TX driver from a blocking one.
    pub fn new(tx: Tx) -> Result<(Self, TxHandlerId), WakerLimitExceededError> {
        let current_index = TX_INDEX.fetch_add(1, Ordering::Relaxed);
        if current_index >= MAX_WAKERS {
            return Err(WakerLimitExceededError);
        }
        let id = TxHandlerId(current_index);

        Ok((Self { inner: tx, id }, id))
    }

    /// Asynchronously write data to the UART.
    pub async fn write(&mut self, buf: &[u8]) -> usize {
        if buf.is_empty() {
            return 0;
        }
        let tx_fut = TxFuture::new(self, buf);
        tx_fut.await;
        buf.len()
    }
}

impl embedded_io_async::ErrorType for TxAsynch {
    type Error = Infallible;
}

impl embedded_io_async::Write for TxAsynch {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Ok(self.write(buf).await)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        while self.inner.regs().read_status().txf() {
            core::hint::spin_loop();
        }
        Ok(())
    }
}

/// TX future structure for an asynchronous UART transmission which can be polled.
pub struct TxFuture<'tx> {
    tx: &'tx mut TxAsynch,
}

impl<'tx> TxFuture<'tx> {
    /// Create a new asynchronous future for a write/transmit operation.
    pub fn new(tx: &'tx mut TxAsynch, data: &[u8]) -> Self {
        TX_DONE[tx.id.0].store(false, core::sync::atomic::Ordering::Relaxed);
        tx.inner.disable();
        tx.inner.disable_interrupts();
        critical_section::with(|cs| {
            let context_ref = TX_CONTEXTS[tx.id.0].borrow(cs);
            context_ref.set(TranferContext {
                data_ptr: data.as_ptr(),
                transfer_len: data.len(),
                progress: 1,
            });
            // We checked in our API that the buffer is not empty. The API should ensure that TX
            // is always empty.
            tx.inner.clear_interrupts();
            tx.inner.enable_interrupts();
            tx.inner.enable();
            // It is actually important to write AFTER the UART was enabled.
            tx.inner.write(data[0]).unwrap();
        });

        Self { tx }
    }
}

impl core::future::Future for TxFuture<'_> {
    type Output = usize;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        TX_WAKERS[self.tx.id.0].register(cx.waker());
        if TX_DONE[self.tx.id.0].swap(false, core::sync::atomic::Ordering::Relaxed) {
            let progress = critical_section::with(|cs| {
                let context = TX_CONTEXTS[self.tx.id.0].borrow(cs);
                let progress = context.get().progress;
                context.set(TranferContext::new());
                progress
            });
            return core::task::Poll::Ready(progress);
        }
        core::task::Poll::Pending
    }
}

impl Drop for TxFuture<'_> {
    fn drop(&mut self) {
        if !TX_DONE[self.tx.id.0].load(core::sync::atomic::Ordering::Relaxed) {
            self.tx.inner.clear_interrupts();
            self.tx.inner.disable_interrupts();
        }
    }
}
