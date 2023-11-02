#![no_std]
#![no_main]

mod io;
mod mm;
mod panic;

extern crate alloc;

use io::uart::{uart_init, uart_write};

#[no_mangle]
pub extern "C" fn main() {
    uart_init();
    uart_write("UART initialization finished!");
    uart_write("hello world!");
    loop {}
}
