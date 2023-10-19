use crate::io::gpio::GPIO;
use crate::io::mmio::MMIO;
use crate::io::periph::*;

pub struct AUX {
    base: MMIO,
}

#[inline(always)]
fn baud(baud: u32) -> u32 {
    AUX_UART_CLOCK / (8 * baud) - 1
}

impl AUX {
    pub fn new(b: u64) -> Self {
        AUX { base: b.into() }
    }

    pub fn write_enable(&self, val: u32) {
        self.base.clone().offset(4 as u64).write(val.into());
    }

    pub fn write_io(&self, val: u32) {
        self.base.clone().offset(64 as u64).write(val.into());
    }

    pub fn write_ier(&self, val: u32) {
        self.base.clone().offset(68 as u64).write(val.into());
    }

    pub fn write_iir(&self, val: u32) {
        self.base.clone().offset(72 as u64).write(val.into());
    }

    pub fn write_lcr(&self, val: u32) {
        self.base.clone().offset(76 as u64).write(val.into());
    }

    pub fn write_mcr(&self, val: u32) {
        self.base.clone().offset(80 as u64).write(val.into());
    }

    pub fn write_lsr(&self, val: u32) {
        self.base.clone().offset(84 as u64).write(val.into());
    }

    pub fn read_lsr(&self) -> u32 {
        self.base.clone().offset(84 as u64).read_u32()
    }

    pub fn write_cntl(&self, val: u32) {
        self.base.clone().offset(96 as u64).write(val.into());
    }

    pub fn write_baud(&self, val: u32) {
        self.base.clone().offset(104 as u64).write(val.into());
    }
}

pub struct UART {
    gpio: GPIO,
    aux: AUX,
}

impl UART {
    pub fn new(gpio: GPIO, aux: AUX) -> Self {
        aux.write_enable(1);
        aux.write_ier(0);
        aux.write_cntl(0);
        aux.write_lcr(3);
        aux.write_mcr(0);
        aux.write_ier(0);
        aux.write_iir(0xC6);
        aux.write_baud(baud(115200));
        gpio.alt(14);
        gpio.alt(15);
        UART { gpio, aux }
    }

    fn ready(&self) -> bool {
        return (self.aux.read_lsr() & 0x20) > 0;
    }

    fn block(&self) {
        while !self.ready() {}
    }

    fn write(&self, ch: u8) {
        self.block();
        self.aux.write_io(ch as u32);
    }

    pub fn write_text(&self, text: &str) {
        for ch in text.bytes() {
            if ch == '\n' as u8 {
                self.write('\r' as u8)
            }
            self.write(ch);
        }
    }
}
