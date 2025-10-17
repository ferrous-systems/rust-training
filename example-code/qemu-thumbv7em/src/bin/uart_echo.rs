//! Print to the UART on an MPS2-AN505

#![no_std]
#![no_main]

use core::cell::RefCell;
use defmt_semihosting as _;

use heapless::spsc::{Producer, Queue};
use qemu_thumbv7em::{interrupt, interrupts::Interrupts, uart, SYSTEM_CLOCK};

const QUEUE_SIZE: usize = 16;
static QUEUE: static_cell::ConstStaticCell<Queue<u8, QUEUE_SIZE>> =
    static_cell::ConstStaticCell::new(Queue::<u8, QUEUE_SIZE>::new());

static RX: critical_section::Mutex<RefCell<Option<uart::Rx>>> =
    critical_section::Mutex::new(RefCell::new(None));
static PRODUCER: critical_section::Mutex<RefCell<Option<Producer<'_, u8, QUEUE_SIZE>>>> =
    critical_section::Mutex::new(RefCell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Running UART echo app. Connect on telnet 4321 to chat!");

    let peripherals = qemu_thumbv7em::Peripherals::take().unwrap();
    let mut uart0 = uart::Uart::new(peripherals.uart0);
    uart0.init(115200, SYSTEM_CLOCK).unwrap();
    let (mut tx, mut rx) = uart0.split();
    rx.enable_interrupts();
    let queue = QUEUE.take();
    let (prod, mut cons) = queue.split();
    critical_section::with(|cs| {
        PRODUCER.borrow(cs).replace(Some(prod));
        RX.borrow(cs).replace(Some(rx));
    });
    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupts::Uart0Rx);
        cortex_m::interrupt::enable();
    }

    loop {
        if let Some(byte) = cons.dequeue() {
            tx.write_blocking(byte);
        }
    }
}

/// Called when UART0 has a RX interrupt
#[interrupt]
fn Uart0Rx() {
    let opt_byte = critical_section::with(|cs| {
        let mut rx_ref = RX.borrow(cs).borrow_mut();
        let rx = rx_ref.as_mut().unwrap();
        let byte = rx.read();
        rx.clear_interrupts();
        byte
    });
    if let Ok(byte) = opt_byte {
        critical_section::with(|cs| {
            if let Some(prod) = PRODUCER.borrow(cs).borrow_mut().as_mut() {
                prod.enqueue(byte).unwrap();
            }
        });
    }
}
