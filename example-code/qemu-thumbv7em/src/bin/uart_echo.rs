//! Print to the UART on an MPS2-AN505

#![no_std]
#![no_main]

use core::cell::RefCell;
use defmt_semihosting as _;

use heapless::spsc::{Producer, Queue};
use qemu_thumbv7em::{
    interrupt,
    interrupts::Interrupts,
    uart::{self, UART0_ADDR},
    SYSTEM_CLOCK,
};

const QUEUE_SIZE: usize = 16;
static QUEUE: static_cell::ConstStaticCell<Queue<u8, QUEUE_SIZE>> =
    static_cell::ConstStaticCell::new(Queue::<u8, QUEUE_SIZE>::new());

static PRODUCER: critical_section::Mutex<RefCell<Option<Producer<'_, u8, QUEUE_SIZE>>>> =
    critical_section::Mutex::new(RefCell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Running UART echo app. Connect on telnet 4321 to chat!");

    let peripherals = qemu_thumbv7em::Peripherals::take().unwrap();
    let mut uart0 = uart::CmsdkUart::new(peripherals.uart0);
    uart0.init(115200, SYSTEM_CLOCK).unwrap();
    uart0.enable_rx_interrupt();
    let queue = QUEUE.take();
    let (prod, mut cons) = queue.split();
    critical_section::with(|cs| {
        PRODUCER.borrow(cs).replace(Some(prod));
    });
    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupts::Uart0Rx);
        cortex_m::interrupt::enable();
    }

    loop {
        if let Some(byte) = cons.dequeue() {
            uart0.write_blocking(byte);
        }
    }
}

/// Called when UART0 has a RX interrupt
#[interrupt]
fn Uart0Rx() {
    let mut uart0 = unsafe { uart::CmsdkUart::new_with_raw_addr(UART0_ADDR) };
    let int_status = uart0.read_int_status();
    if let Ok(byte) = uart0.read() {
        critical_section::with(|cs| {
            if let Some(prod) = PRODUCER.borrow(cs).borrow_mut().as_mut() {
                prod.enqueue(byte).unwrap();
            }
        });
    }
    uart0.clear_interrupts(int_status);
}
