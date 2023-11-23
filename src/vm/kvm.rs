use alloc::vec::Vec;
use crate::kalloc::pages::PageTableEntry;
use crate::memlayout::{MAXVA, KERNBASE, PHYSTOP};
use crate::riscv::pteflags::{PTE_V, PTE_R, PTE_X, PTE_W};
use crate::riscv::{pgrounddown,pgroundup, PGSIZE};
use crate::kalloc::{pagealloc::{PageGurd, PAGE_ALLOCER}, pages::{PAddress, VAddress}};

use super::{VMError, get_idx};

//直接映射
pub struct Kvm{
    pagetable: PageGurd,
    paddr: Vec<PageGurd>,
}
//TODO test kvm mappages and walk
impl Kvm {
    pub fn new() -> Self{
        let page = PAGE_ALLOCER.alloc();
        Self {
            pagetable: page,
            paddr: Vec::new(), 
        }
    }

    pub fn kvmmake(&mut self){
        extern "C"{
            fn stext();
            fn etext();
            fn srodata();
            fn erodata();
            fn sdata();
            fn edata();
            fn ekernel();
            fn skernel();
        }

        self.mappages(
            KERNBASE.into(), 
            KERNBASE.into(), 
            (etext as usize) - KERNBASE, 
            PTE_R | PTE_X | PTE_W
        );
        info!("run here");
        self.mappages(
            ((etext as usize)).into(),
            ((etext as usize)).into(),
            PHYSTOP-(etext as usize), 
            PTE_R|PTE_X|PTE_W
        );
    }

    pub fn as_satp(&self) -> usize {
        let pa = self.pagetable.to_pa().0;
        let sv_39 = 8 ; 
        (sv_39 << 60) | (pa >> 12)
    }

    fn save_page(&mut self,page:PageGurd){
        self.paddr.push(page);
    }
    pub fn walk(
        &mut self,
        va: usize,
        alloc: bool,
        perm: usize
    ) -> Result<&mut PageTableEntry,VMError>{
        if va > MAXVA {
            panic!("va to big")
        }
        let mut pagetable = 
            self.pagetable.inner.get_pte_array();
        for level in (1..3).rev() {
            let pte = &mut pagetable[get_idx(va, level)];
            if pte.is_v() {
                pagetable = pte.to_pa().get_pte_array();
            } else {
                if !alloc {
                    return Err(VMError::NotAlloc);
                } else {
                    let page = PAGE_ALLOCER.alloc();
                    let pa = (*page).to_pa();
                    pte.set(pa.0, PTE_V);
                    pagetable = pa.get_pte_array();
                    self.save_page(page);
                }
            }
        }
        let pte = &mut pagetable[get_idx(va, 0)];
        Ok(pte)
        
    }

    pub fn mappages(
        &mut self,
        va: VAddress,
        pa: PAddress,
        sz: usize,
        perm: usize
    ) -> Result<(),VMError>{
        if sz == 0 {
            panic!("Error sz == 0 !!")
        }

        let mut a = pgrounddown(va.0);
        let mut last = pgrounddown(va.0+sz -1 );
        let mut pa = pa.0;
        loop {
            match self.walk(a, true, perm) {
                Ok(pte) => {
                    // info!("run here {:p}",pte);
                    if pte.is_v() {
                        panic!("mappages:remap");
                    }
                    pte.set(pa, perm | PTE_V);
                    
                    if a == last {
                        break Ok(());
                    }
                    a += PGSIZE;
                    pa += PGSIZE;
                }
                Err(e) => {
                    break Err(e);
                }
            }
        } 
    }
}