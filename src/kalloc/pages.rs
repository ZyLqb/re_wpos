use alloc::sync::Arc;
use crate::riscv::{sv39,pgrounddown,pgroundup, PGSIZE};

#[derive(Clone,Copy,Ord,PartialEq, PartialOrd,Eq)]
pub struct PAddress(pub usize);

#[derive(Clone,Copy,Ord,PartialEq, PartialOrd,Eq)]
pub struct VAddress(pub usize);

#[derive(Clone,Copy,Ord,PartialEq, PartialOrd,Eq)]
pub struct PPageNum(pub usize);

#[derive(Clone,Copy,Ord,PartialEq, PartialOrd,Eq)]
pub struct VPageNum(pub usize);

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
    pub fn to_addr(self) -> PAddress{
        let a: PAddress = (self.0 * PGSIZE).into();
        a
        
    }
    pub fn add_one(&self) -> PPageNum{
        PPageNum(self.0 + 1)
    }

    pub fn get_pte_array(&self) -> &'static mut [PPageNum]{
        self.to_addr().get_pte_array()
    }

    pub fn get_bytes_array(&self) -> &'static mut [u8]{
        self.to_addr().get_bytes_array()
    }

    pub fn get_mut<T>(&self) -> &'static mut T{
        let pa: PAddress = self.to_addr();
        unsafe{ 
            (pa.0 as *mut T).as_mut().unwrap()
        }
    }
}

impl PAddress {

    pub fn get_pte_array(&self) -> &'static mut [PPageNum] {
        let pa: usize = self.clone().into();
        let pa: PAddress = pa.into();
        unsafe{
            core::slice::from_raw_parts_mut(
                pa.0 as *mut PPageNum,
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