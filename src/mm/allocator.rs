use buddy_system_allocator::LockedHeap;
#[global_allocator]
pub static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::new();
pub static mut HEAP: [u8; 65536] = [0; 65536];

#[inline(always)]
pub fn heap_init() {
    unsafe{
    HEAP_ALLOCATOR
        .lock()
        .init(HEAP.as_mut_ptr() as usize, HEAP.len());
    }
}
