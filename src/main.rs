#![no_std]
#![no_main]

mod mm;
mod lib;
mod panic;
mod periph;

extern crate alloc;
extern crate spin;

use crate::periph::{uart_init, uart_write};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::lib::stdout(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[no_mangle]
pub extern "C" fn main() {
    uart_init();
    uart_write("UART initialization finished!\n");
    // FIXME: Really Don't know why the locking fails here. Needs further digging.
    println!("hello world!");
    loop {}
}

