use num_traits::{PrimInt, Unsigned};

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

    pub fn offset<T>(mut self, offset: T) -> Self
    where
        T: TryInto<u64>,
    {
        // TODO: Check i64/u64 num_size
        self.addr += offset.try_into().unwrap_or(0);
        self
    }

    pub fn write<T>(&self, val: T)
    where
        T: PrimInt,
    {
        unsafe {
            core::ptr::write_volatile(self.addr as *mut T, val);
        }
    }

    pub fn write_w_off<T, U>(&self, offset: T, val: U)
    where
        T: TryInto<u64>,
        U: PrimInt,
    {
        self.clone().offset(offset).write(val);
    }

    pub fn read<T>(&self) -> T
    where
        T: Unsigned,
    {
        unsafe { core::ptr::read_volatile(self.addr as *const T) }
    }

    pub fn read_w_off<T, U>(&self, offset: T) -> U
    where
        T: TryInto<u64>,
        U: Unsigned,
    {
        self.clone().offset(offset).read()
    }
}

impl From<u64> for MMIO {
    fn from(value: u64) -> Self {
        MMIO { addr: value }
    }
}
