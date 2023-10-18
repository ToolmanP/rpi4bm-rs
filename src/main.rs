#![no_std]
#![no_main]

extern crate alloc;

mod io;
mod mm;
mod panic;

use io::gpio::GPIO;
use io::periph::*;
use io::uart::*;
use mm::allocator::heap_init;

#[no_mangle]
pub extern "C" fn main() {

    heap_init();

    let gpio = GPIO::new(GPSET, GPCLR, GPPUD, GPFSEL);
    let aux = AUX::new(UART_AUX_BASE);
    let uart = UART::new(gpio, aux);

    for _ in 0..10 {
        uart.write_text("Hello world!\n");
    }

    loop {}
}
