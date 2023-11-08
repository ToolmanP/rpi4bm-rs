use buddy_system_allocator::LockedHeap;
use crate::println;

const HEAP_SIZE: usize = 10 * 1024 * 4096; // 40MB
//
#[repr(align(4096))]
struct HeapRegion([u8; HEAP_SIZE]);

static mut HEAP_REGION: HeapRegion = HeapRegion([0; HEAP_SIZE]);

#[global_allocator]
pub static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

pub fn heap_init() {
    unsafe {
        println!("init buddy system");
        unsafe {
            HEAP_ALLOCATOR
                .lock()
                .init(HEAP_REGION.0.as_mut_ptr() as usize, HEAP_SIZE);
        }
    }
}
