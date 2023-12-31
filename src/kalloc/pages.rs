use alloc::sync::Arc;
use crate::riscv::{sv39,pgrounddown,pgroundup, PGSIZE,pteflags::*};

#[derive(Clone,Copy,Ord,PartialEq, PartialOrd,Eq)]
pub struct PAddress(pub usize);

#[derive(Clone,Copy,Ord,PartialEq, PartialOrd,Eq)]
pub struct VAddress(pub usize);

#[derive(Clone,Copy,Ord,PartialEq, PartialOrd,Eq)]
pub struct PPageNum(pub usize);

#[derive(Clone,Copy,Ord,PartialEq, PartialOrd,Eq)]
pub struct VPageNum(pub usize);
#[derive(Clone,Copy,Ord,PartialEq, PartialOrd,Eq)]
pub struct PageTableEntry(pub usize);

impl From<usize> for PAddress {
    fn from(value: usize) -> Self {
        Self(value & ((1 << sv39::PA_WIDTH) - 1))
    }
}
impl From<PAddress> for usize {
    fn from(value: PAddress) -> Self {
        value.0
    }
}
impl From<usize> for VAddress {
    fn from(value: usize) -> Self {
        Self(value & ((1 << sv39::VA_WIDTH) - 1))
    }
}

impl From<VAddress> for usize {
    fn from(value: VAddress) -> Self {
        value.0
    }
}

impl From<PAddress> for PPageNum {
    fn from(value: PAddress) -> Self {
        Self (
            pgrounddown(value.into()) / PGSIZE
        )
    }
}
impl From<VAddress> for VPageNum {
    fn from(value: VAddress) -> Self {
        Self (
            pgrounddown(value.into()) / PGSIZE
        )
    }
}

impl From<usize> for PPageNum {
    fn from(value: usize) -> Self {
        Self (
            pgrounddown(value) / PGSIZE
        )
    }
}


impl PPageNum {
    pub fn to_pa(self) -> PAddress{
        let a: PAddress = (self.0 * PGSIZE).into();
        a
    }
    pub fn add_one(&self) -> PPageNum{
        PPageNum(self.0 + 1)
    }

    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry]{
        self.to_pa().get_pte_array()
    }

    pub fn get_bytes_array(&self) -> &'static mut [u8]{
        self.to_pa().get_bytes_array()
    }

    pub fn get_mut<T>(&self) -> &'static mut T{
        let pa: PAddress = self.to_pa();
        unsafe{ 
            (pa.0 as *mut T).as_mut().unwrap()
        }
    }
}

impl PAddress {

    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: usize = self.clone().into();
        let pa: PAddress = pa.into();
        unsafe{
            core::slice::from_raw_parts_mut(
                pa.0 as *mut PageTableEntry,
                512
            )
        }
    }

    pub fn get_bytes_array(&self) -> &'static mut [u8]{
        let pa: usize = self.clone().into();
        let pa: PAddress = pa.into();
        unsafe{
            core::slice::from_raw_parts_mut(pa.0 as *mut u8, PGSIZE)
        }
    }

    pub fn get_mut<T>(&self) -> &'static mut T{
        let pa: usize = self.clone().into();
        let pa: PAddress = pa.into();
        unsafe{ 
            (pa.0 as *mut T).as_mut().unwrap()
        }
    }
}

impl PageTableEntry{
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: usize = self.0;
        let pa: PAddress = pa.into();
        unsafe{
            core::slice::from_raw_parts_mut(
                pa.0 as *mut PageTableEntry,
                512
            )
        }
    }
   
    pub fn get_mut<T>(&self) -> &'static mut T{
        let pa: usize = self.0;
        let pa: PAddress = pa.into();
        //println!("pa : {:#x}",pa.0);
        unsafe{ 
            (pa.0 as *mut T).as_mut().unwrap()
        }
    }

    pub fn set(&mut self, pa: usize, prem: usize) {
        //pa to pte
        self.0 = ((pa >> 12) << 10) | prem;
    }

    pub fn is_v(&self) -> bool {
        self.0 & PTE_V != 0
    }

    pub fn flags(&self) -> usize {
        self.0 & 0x3FF
    }

    pub fn to_pa(&self) -> PAddress {
        ((self.0 >> 10) << 12).into()
    }
}