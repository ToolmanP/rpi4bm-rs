use crate::io::mmio::MMIO;
use crate::io::periph::*;

struct Base {
    io: MMIO,
    sz: u32,
    max: u32,
}

impl Base {
    pub fn write(&self, pin: u32, val: u32) {
        let mask: u32 = (1 << self.sz) - 1;
        let num_fields = 32 / self.sz;
        let off = (pin / num_fields) << 2;
        let shift = (pin % num_fields) * self.sz;
        let reg = self.io.clone().offset(off as u64);
        let mut curval = reg.read_u32();

        curval &= !(mask << shift);
        curval |= val << shift;
        reg.write(curval.into());
    }
}

pub struct GPIO {
    set: Base,
    clr: Base,
    pup: Base,
    sel: Base,
}

impl GPIO {
    pub fn new(st: u64, c: u64, p: u64, sl: u64) -> Self {
        GPIO {
            set: Base {
                io: MMIO::new(st),
                sz: 1,
                max: GPIO_MAX_PIN,
            },
            clr: Base {
                io: MMIO::new(c),
                sz: 1,
                max: GPIO_MAX_PIN,
            },
            pup: Base {
                io: MMIO::new(p),
                sz: 2,
                max: GPIO_MAX_PIN,
            },
            sel: Base {
                io: MMIO::new(sl),
                sz: 3,
                max: GPIO_MAX_PIN,
            },
        }
    }
    pub fn clear(&self, pin: u32, val: u32) {
        self.clr.write(pin, val);
    }

    pub fn function(&self, pin: u32, val: u32) {
        self.sel.write(pin, val)
    }

    pub fn pull(&self, pin: u32, val: u32) {
        self.pup.write(pin, val);
    }

    pub fn set(&self, pin: u32, val: u32) {
        self.set.write(pin, val);
    }

    pub fn alt(&self, pin: u32) {
        self.pull(pin, 0);
        self.function(pin, GPIO_FUNCTION_ALT5);
    }
}
