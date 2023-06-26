pub use crate::riscv::PGSIZE;

pub const KERNBASE: usize = 0x80200000;
pub const PHYSTOP: usize = KERNBASE + 128 * 1024 * 1024 - 0x00200000;


pub const PAGE_SIZE_BITS:usize = 0x0c;
pub const PA_WIDTH_SV39:usize = 56;
pub const VA_WIDTH_SV39:usize = 39; 
pub const PPN_SV39:usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;
pub const VPN_SV39:usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

pub const MAXVA:usize = (1 << (9 + 9 + 9 + 12 - 1));