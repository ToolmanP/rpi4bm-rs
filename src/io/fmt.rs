#[macro_export]
macro_rules! print {
    ($($arg:tt) *) => {
        io::uart::uart_write(alloc::format!($($arg) *).as_str());
    }
}
