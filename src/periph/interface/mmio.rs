use num_traits::{PrimInt, Unsigned};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MMIO {
    addr: u64,
}

// We leverage the num_traits for better abstraction over r/w operations.
impl MMIO {
    pub fn new(addr: u64) -> Self {
        MMIO { addr }
    }

    pub fn offset<T>(mut self, offset: T) -> Self
    where
        T: TryInto<u64>,
    {
        // FIXME: this is a tricky assumption
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
