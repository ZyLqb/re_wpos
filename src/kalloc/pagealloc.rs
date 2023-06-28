
//TODO 写代码注释
use core::ops::Deref;
use alloc::{vec::*, collections::BTreeMap};
use alloc::sync::Arc;
use crate::{lock::SpinLock, memlayout::{PHYSTOP, KERNBASE}, riscv::PGSIZE};
use crate::riscv::{pgrounddown,pgroundup};
use super::pages::{self, PPageNum};
pub static PAGE_ALLOCER:PageAllocer = PageAllocer::new();

pub struct AllPages {
    current: PPageNum,
    end: PPageNum,
    free: Vec<PPageNum>,
    alloced: BTreeMap<usize,Arc<PPageNum>>,
}
#[derive(Clone)]
pub struct PageGurd {
    pub inner:Arc<PPageNum>
}
impl PageGurd {
    fn new(inner:Arc<PPageNum>) -> Self {
        Self { inner }   
    }
}

impl AllPages {
    pub const fn new() -> Self{
        Self { 
            current: PPageNum(0), 
            end: PPageNum(0), 
            free: Vec::new(),
            alloced: BTreeMap::new(),
        }
    }

    pub fn init(&mut self) {
        extern "C" {
            fn ekernel();
        }
        let start = pgroundup(ekernel as usize);
        self.current = PPageNum::from(start);
        self.end = PPageNum::from(PHYSTOP - 100*PGSIZE);
    }

    pub fn alloc(&mut self) -> Arc<PPageNum>{
        if self.current >= self.end{
            //FIXME#4(Who):不应该直接panic
            panic!("no pages")
        }
        let page: Option<PPageNum> = self.free.pop();
        match page{
            None => {
                let now = self.current;
                self.current = self.current.add_one();
                match self.alloced.insert(now.0, Arc::new(now)) {
                    Some(_) => {
                        //Btree里面已经有数据,数据插入未成功
                        let ret_page = self.alloced.get(&(now.0)).unwrap();
                        ret_page.clone()
                    }
                    None => {
                        //数据插入成功
                        let ret_page = self.alloced.get(&(now.0)).unwrap();
                        ret_page.clone()
                    }
                }
            }
            Some(now) => {
                match self.alloced.insert(now.0, Arc::new(now)) {
                    Some(_) => {
                        //Btree里面已经有数据,数据插入未成功
                        let ret_page = self.alloced.get(&(now.0)).unwrap();
                        ret_page.clone()
                    }
                    None => {
                        //数据插入成功
                        let ret_page = self.alloced.get(&(now.0)).unwrap();
                        ret_page.clone()
                    }
                }
            } 
        } 
    }

    pub fn dealloc(&mut self,page:Arc<PPageNum>){
        if *page > self.current {
            let a = self.free.iter().find(|x| **x == *page);
            match a {
                Some(value) => panic!("page : {:#x} not alloced",(*value).to_addr().0),
                None => {}
            }
        }

        if Arc::strong_count(&page) == 3 {
            let rmv_save = self.alloced.remove(&page.0).unwrap().0;
            self.free.push(PPageNum(rmv_save));
            assert_eq!(Arc::strong_count(&page),2);
            //value have one and here have one
            //after this fun the count is 0
            drop(page);
        }else {
            drop(page)
        }
    }
}

pub struct PageAllocer {
    allocer: SpinLock<AllPages>,
}

impl PageAllocer {
    pub const fn new() -> Self {
       Self{
        allocer: SpinLock::new(AllPages::new())
       }
    }

    pub fn alloc(&self) -> PageGurd {
        let page = self.allocer.locked().alloc();
        let byte_arry = page.get_bytes_array();
        for i in byte_arry {
            *i = 0;
        }
        PageGurd { inner:page }
    }

    pub fn dealloc(&self,page:Arc<PPageNum>){
        unsafe{
            self.allocer.locked().dealloc(page);
        }
    }

    pub fn init(&self){
        self.allocer.locked().init();
    }
}

impl Drop for PageGurd {
    fn drop(&mut self) {
        let a = self.inner.clone();
        PAGE_ALLOCER.allocer.locked().dealloc(a);
    }
}

pub fn page_init(){
    PAGE_ALLOCER.init();
}

impl Deref for PageGurd {
    //type Target = T;
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.inner.0
    }
}