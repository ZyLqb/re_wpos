pub use crate::riscv::PGSIZE;

pub const KERNBASE: usize = 0x80200000;
pub const PHYSTOP: usize = KERNBASE + 128 * 1024 * 1024 - 0x00200000;


pub const MAXVA:usize = 1 << (9 + 9 + 9 + 12 - 1);