//目前的cpu模块还比较简单，定一个cpu数组，有NCPU个位置
//目前的主要的作用是让cpu在被锁的时候禁止中断
//之后在进程切换的时候会用到context和proc

use core::cell::UnsafeCell;
use core::arch::asm;
pub const NCPU:usize = 3;

pub static CPUS :Cpus = Cpus::new();

use crate::{riscv::{intr_off,intr_on, intr_get}, array};
//Cpu结构体，intena表示当前状态下，中断是否被打开
//noff表示中断被禁用的次数,dang noff == 0 时Cpu才打开中断
pub struct Cpu{
    //pub proc: 
    //pub context;
    pub intena: bool,
    pub noff: UnsafeCell<usize>,
}

//Cpus 里面封装了一个Cpu数组
pub struct Cpus([UnsafeCell<Cpu>;NCPU]);
unsafe impl  Sync for Cpus{}
//cpu 中断锁的守护结构体
pub struct IntrLockGurd<'a>{
    cpu:&'a Cpu
}

impl Cpu {
    pub const fn new() -> Self{
        Self { 
            intena: true, 
            noff: UnsafeCell::new(0) 
        }
    }

    fn locked(&mut self,old:bool) -> IntrLockGurd {
        intr_off();
        unsafe{
            if *self.noff.get() == 0 {
                self.intena = old;
            }
            *self.noff.get() += 1;
        }
        IntrLockGurd { cpu: self }
    }

    unsafe fn unlock(&self){
        let noff = self.noff.get();
        assert!(!intr_get(),"cpu unlock: intr on!");
        //assert!(*noff >= 1, "cpu unlock: cpu is intr on!");        
        *noff -= 1;
        if *noff == 0 && self.intena {
            intr_on();
        }
    }
}

impl Cpus {
    pub const fn new() -> Self{
        Self(array![ UnsafeCell::new(Cpu::new()) ; NCPU ])
    }
    #[inline]
    pub unsafe fn cpu_id() -> usize {
        let id;
        asm!("mv {0}, tp", out(reg) id);
        id
    }

    pub fn my_cpu(&self) -> &mut Cpu{
        unsafe{
            let id = Self::cpu_id();
            &mut *self.0[id].get()
        }
    }
    pub fn intr_lock(&self) -> IntrLockGurd {
        let old = intr_get();
        self.my_cpu().locked(old)
    }
}
impl <'a>Drop for IntrLockGurd<'a> {
    fn drop(&mut self){
        unsafe{
            self.cpu.unlock();
        }
    }
}
