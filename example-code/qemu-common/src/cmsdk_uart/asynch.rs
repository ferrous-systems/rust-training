use core::{cell::RefCell, convert::Infallible, sync::atomic::AtomicBool};

use atomic_waker::AtomicWaker;
use critical_section::Mutex;

use crate::cmsdk_uart::Tx;

const MAX_WAKERS: usize = 5;

static TX_WAKERS: [AtomicWaker; 5] = [const { AtomicWaker::new() }; 5];
static TX_DONE: [AtomicBool; 5] = [const { AtomicBool::new(false) }; 5];
static TX_CONTEXTS: [Mutex<RefCell<TranferContext>>; 5] =
    [const { Mutex::new(RefCell::new(TranferContext::new())) }; 5];

#[derive(Debug)]
pub struct InvalidWakerIndex(pub usize);

pub fn on_interrupt_tx(tx: &mut Tx, uart_index: usize) {
    if uart_index >= MAX_WAKERS {
        return;
    }
    tx.clear_interrupts();
    if !tx.regs().read_control().txie() || tx.regs().read_status().txf() {
        return;
    }
    let mut context = critical_section::with(|cs| {
        let context_ref = TX_CONTEXTS[uart_index].borrow(cs);
        *context_ref.borrow()
    });
    // No transfer active.
    if context.data_ptr.is_null() {
        return;
    }
    let slice = unsafe { core::slice::from_raw_parts(context.data_ptr, context.transfer_len) };
    let slice_len = slice.len();
    if context.progress >= slice_len || slice_len == 0 {
        // Transfer is done. Notify executor and set completion flag.
        TX_DONE[uart_index].store(true, core::sync::atomic::Ordering::Relaxed);
        TX_WAKERS[uart_index].wake();
        tx.disable_interrupts();
        return;
    }

    // Write next byte of transfer.
    tx.write(slice[context.progress]).unwrap();
    context.progress += 1;

    // Write back updated context structure.
    critical_section::with(|cs| {
        let context_ref = TX_CONTEXTS[uart_index].borrow(cs);
        *context_ref.borrow_mut() = context;
    });
}

#[derive(Clone, Copy)]
struct TranferContext {
    data_ptr: *const u8,
    transfer_len: usize,
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

// We only use this type wrapped in a mutex.
unsafe impl Sync for TranferContext {}
unsafe impl Send for TranferContext {}

pub struct TxAsynch {
    inner: Tx,
    uart_index: usize,
}

impl TxAsynch {
    /// Create a new asynchronous TX driver from a blocking one.
    pub fn new(tx: Tx, uart_index: usize) -> Result<Self, InvalidWakerIndex> {
        if uart_index > 4 {
            return Err(InvalidWakerIndex(uart_index));
        }

        Ok(Self {
            inner: tx,
            uart_index,
        })
    }

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
    /// It is very important that the data which is sent outlives the asynchronous futures
    /// created with it.
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Ok(self.write(buf).await)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub struct TxFuture<'tx> {
    tx: &'tx mut TxAsynch,
    uart_index: usize,
}

impl<'tx> TxFuture<'tx> {
    pub fn new(tx: &'tx mut TxAsynch, data: &[u8]) -> Self {
        let uart_index = tx.uart_index;
        TX_DONE[uart_index].store(false, core::sync::atomic::Ordering::Relaxed);
        tx.inner.disable();
        tx.inner.disable_interrupts();
        critical_section::with(|cs| {
            let mut context = TX_CONTEXTS[uart_index].borrow_ref_mut(cs);
            context.data_ptr = data.as_ptr();
            context.transfer_len = data.len();
            context.progress = 1;
            // We checked in our API that the buffer is not empty. The API should ensure that TX
            // is always empty.
            tx.inner.clear_interrupts();
            tx.inner.enable_interrupts();
            tx.inner.enable();
            // It is actually important to write AFTER the UART was enabled.
            tx.inner.write(data[0]).unwrap();
        });

        Self { tx, uart_index }
    }
}

impl core::future::Future for TxFuture<'_> {
    type Output = usize;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        TX_WAKERS[self.uart_index].register(cx.waker());
        if TX_DONE[self.uart_index].swap(false, core::sync::atomic::Ordering::Relaxed) {
            let progress = critical_section::with(|cs| {
                let mut ctx = TX_CONTEXTS[self.uart_index].borrow(cs).borrow_mut();
                ctx.data_ptr = core::ptr::null();
                ctx.transfer_len = 0;
                ctx.progress
            });
            return core::task::Poll::Ready(progress);
        }
        core::task::Poll::Pending
    }
}

impl Drop for TxFuture<'_> {
    fn drop(&mut self) {
        if !TX_DONE[self.uart_index].load(core::sync::atomic::Ordering::Relaxed) {
            self.tx.inner.clear_interrupts();
            self.tx.inner.disable_interrupts();
        }
    }
}
