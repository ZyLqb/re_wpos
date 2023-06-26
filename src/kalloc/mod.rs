pub mod page_alloc;


use slab_allocator_rs::LockedHeap;

use crate::riscv::PGSIZE;
const MIN_HEAP:usize = PGSIZE *8;
const HEAP_SIZE:usize = 0x300_000;
//FIXME:#3(Who):必须要与0x8000对齐，浪费了0x16_000的空间
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub static mut HEAP_SPACE: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

pub fn init_heap() {
    let mut heap_start =unsafe{HEAP_SPACE.as_ptr() as usize};
    
    let heap_end =( heap_start+HEAP_SIZE) & !(MIN_HEAP-1);
    heap_start = (heap_start+(MIN_HEAP-1)) / MIN_HEAP * MIN_HEAP;
    
    let heap_size = heap_end-heap_start;

    unsafe {
        ALLOCATOR.init(heap_start, heap_size);
    }
}