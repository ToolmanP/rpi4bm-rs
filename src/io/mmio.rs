#[repr(C)]
#[derive(Clone, Copy)]
pub struct MMIO {
    addr: u64,
}

pub enum Raw {
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
}

impl From<u64> for Raw {
    fn from(val: u64) -> Self {
        Raw::U64(val)
    }
}

impl From<u32> for Raw {
    fn from(val: u32) -> Self {
        Raw::U32(val)
    }
}

impl From<u16> for Raw {
    fn from(val: u16) -> Self {
        Raw::U16(val)
    }
}

impl From<u8> for Raw {
    fn from(val: u8) -> Self {
        Raw::U8(val)
    }
}

impl MMIO {
    pub fn new(addr: u64) -> Self {
        MMIO { addr }
    }

    pub fn offset(mut self, off: u64) -> Self {
        self.addr += off;
        self
    }

    pub fn write(&self, val: Raw) {
        unsafe {
            match val {
                Raw::U64(u) => *(self.addr as *mut u64) = u,
                Raw::U32(u) => *(self.addr as *mut u32) = u,
                Raw::U16(u) => *(self.addr as *mut u16) = u,
                Raw::U8(u) => *(self.addr as *mut u8) = u,
            }
        }
    }

    pub fn read_u8(&self) -> u8 {
        unsafe {
            return *(self.addr as *const u8);
        }
    }

    pub fn read_u16(&self) -> u16 {
        unsafe {
            return *(self.addr as *const u16);
        }
    }

    pub fn read_u32(&self) -> u32 {
        unsafe {
            return *(self.addr as *const u32);
        }
    }

    pub fn read_u64(&self) -> u64 {
        unsafe {
            return *(self.addr as *const u64);
        }
    }
}

impl From<u64> for MMIO {
    fn from(value: u64) -> Self {
        MMIO { addr: value }
    }
}
