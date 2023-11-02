use buddy_system_allocator::LockedHeap;

use crate::io::uart::uart_write;

#[global_allocator]
pub static HEAP_ALLOCATOR: LockedHeap<64> = LockedHeap::<64>::new();
pub static mut HEAP: [u8; 65536] = [0; 65536];

pub fn heap_init() {
    unsafe {
        if let Some (mut heap) = HEAP_ALLOCATOR.try_lock(){
            heap.init(HEAP.as_mut_ptr() as usize, HEAP.len());
        } else {
            uart_write("HEAP LOCK FAILED");
        }
    }
}
