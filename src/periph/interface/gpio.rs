use crate::periph::interface::MMIO;

pub const GPIO_MAX_PIN: u32 = 53;
pub const GPIO_PULL_NONE: u32 = 0;
pub const GPIO_FUNCTION_ALT5: u32 = 2;

struct Set{
    io: MMIO,
    sz: u32,
    max: u32,
}

impl Set {
    pub fn write_pin(&self, pin: u32, val: u32) {
        let mask: u32 = (1 << self.sz) - 1;
        let num_fields: u32 = 32 / self.sz;
        let off: u32 = (pin / num_fields) << 2;
        let shift = (pin % num_fields) * self.sz;
        let mut curval: u32 = self.io.read_w_off(off);

        curval &= !(mask << shift);
        curval |= val << shift;
        self.io.write_w_off(off, curval);
    }
}

pub struct GPIO {
    set: Set,
    clr: Set,
    pup: Set,
    sel: Set,
}

impl GPIO {
    pub fn new(set: u64, clr: u64, pup: u64, sel: u64) -> Self {
        GPIO {
            set: Set {
                io: MMIO::new(set),
                sz: 1,
                max: GPIO_MAX_PIN,
            },
            clr: Set {
                io: MMIO::new(clr),
                sz: 1,
                max: GPIO_MAX_PIN,
            },
            pup: Set {
                io: MMIO::new(pup),
                sz: 2,
                max: GPIO_MAX_PIN,
            },
            sel: Set {
                io: MMIO::new(sel),
                sz: 3,
                max: GPIO_MAX_PIN,
            },
        }
    }
    pub fn clear(&self, pin: u32, val: u32) {
        self.clr.write_pin(pin, val);
    }

    pub fn function(&self, pin: u32, val: u32) {
        self.sel.write_pin(pin, val)
    }

    pub fn pull(&self, pin: u32, val: u32) {
        self.pup.write_pin(pin, val);
    }

    pub fn set(&self, pin: u32, val: u32) {
        self.set.write_pin(pin, val);
    }

    pub fn alt5(&self, pin: u32) {
        self.pull(pin, GPIO_PULL_NONE);
        self.function(pin, GPIO_FUNCTION_ALT5);
    }
}
