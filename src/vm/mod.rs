use alloc::vec::Vec;
use crate::lock::OnceLock;
use crate::riscv::{self, sfence_vma};
use crate::{kalloc::pagealloc::{PageGurd, PAGE_ALLOCER}, riscv::PGSHIFT};
pub static KVM:OnceLock<Kvm> = OnceLock::new();
pub mod kvm;
pub mod uvm;

pub use kvm::Kvm;

pub enum VMError {
    MapError,
    WalkError,
    NotAlloc,
}
pub enum Section {
    DATA,
    OTHER,
}

fn get_idx(vaddr: usize,level:usize) -> usize {
    (vaddr >> (12 + 9*level )) & 0x1ff
}

pub fn k_init(){
    KVM.set(Kvm::new());
    KVM.get_mut().unwrap().kvmmake();
}

pub fn kvminithart(){
    let satp = KVM.get().unwrap().as_satp();
    unsafe{
        riscv::registers::satp::write(satp);
        sfence_vma();
    }
}

