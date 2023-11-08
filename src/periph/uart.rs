use crate::periph::interface::MMIO;
use crate::periph::interface::GPIO;
use crate::periph::base::*;

use core::fmt::Write;

pub const AUX_UART_CLOCK: u32 = 500000000;

pub struct AUX {
    io: MMIO,
}

impl AUX {
    pub fn new(base: u64) -> Self {
        AUX { io: base.into() }
    }

    pub fn write_enable(&self, val: u32) {
        self.io.write_w_off(4, val);
    }

    pub fn write_io(&self, val: u32) {
        self.io.write_w_off(64, val);
    }

    pub fn write_ier(&self, val: u32) {
        self.io.write_w_off(68, val);
    }

    pub fn write_iir(&self, val: u32) {
        self.io.write_w_off(72, val);
    }

    pub fn write_lcr(&self, val: u32) {
        self.io.write_w_off(76, val);
    }

    pub fn write_mcr(&self, val: u32) {
        self.io.write_w_off(80, val);
    }

    pub fn read_lsr(&self) -> u32 {
        self.io.read_w_off(84)
    }

    pub fn write_cntl(&self, val: u32) {
        self.io.write_w_off(96, val);
    }

    pub fn write_baud(&self, val: u32) {
        self.io.write_w_off(104, AUX_UART_CLOCK / (8 * val) - 1);
    }
}

pub struct UART {
    aux: AUX,
}

impl UART {
    pub fn new(gp: GPIO, aux: AUX) -> Self {
        aux.write_enable(1);
        aux.write_ier(0);
        aux.write_cntl(0);
        aux.write_lcr(3);
        aux.write_mcr(0);
        aux.write_ier(0);
        aux.write_iir(0xC6);
        aux.write_baud(115200);
        gp.alt5(14);
        gp.alt5(15);
        aux.write_cntl(3);
        UART { aux }
    }

    fn ready(&self) -> bool {
        return (self.aux.read_lsr() & 0x20) != 0;
    }

    fn block(&self) {
        while !self.ready() {}
    }

    fn write(&self, ch: u8) {
        self.block();
        self.aux.write_io(ch as u32);
    }
}

impl Write for UART {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.bytes() {
            if ch == '\n' as u8 {
                self.write('\r' as u8)
            }
            self.write(ch);
        }
        Ok(())
    }
}

static mut RASPI_UART: Option<UART> = None;
pub fn uart_init() {
    let gpio = GPIO::new(GPSET0, GPCLR0, GPPUPPDN0, GPFSEL0);
    let aux = AUX::new(AUX_BASE);
    unsafe {
        RASPI_UART = Some(UART::new(gpio, aux));
    }
}

pub fn uart_write(s: &str){
    unsafe{
        RASPI_UART.as_mut().unwrap().write_str(s).unwrap(); 
    }
}
