use crate::periph::uart_write;
use core::fmt::Write;

use spin::Mutex;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        uart_write(s);
        Ok(())
    }
}

static STDOUT : Mutex<Stdout> = Mutex::new(Stdout);

pub fn stdout(args: core::fmt::Arguments) {
    STDOUT.lock().write_fmt(args).unwrap();
}
