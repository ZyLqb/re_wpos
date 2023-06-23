#![allow(unused)]
//自旋锁Spinlock
//主要由两个结构体组合完成完整的功能
//第一个是锁的本体：Spinlock，第二个是自旋锁的守护结构体拍：SpinLockGurd
//此自旋锁实现了RALL特性，无需手动释放锁

/// # Example
/// ```
/// fn main(){
///    let vec:Vec<i32> = Vec::new();
///    let vec_lock = SpinLock::new(vec);
///    {
///        let mut gurd = vec_lock.locked();
///        *gurd = *gurd.push(1);
///    }
/// }
/// ```
// 通过locked（）获取SpinLockGurd 
// SpinLockGurd实现了解引用的trait是的我们可以直接访问数据


use core::{ptr,sync::atomic::{Ordering,AtomicPtr}, cell::UnsafeCell};
use crate::process::cpu::{CPUS,Cpu,IntrLockGurd};

//FIXME#1(Who): 当线程panic之后应该释放锁，
//让其他线程可以访问数据
//可以看看Rust std库里面的Mutex实现
//或者可以看看spin::Mutex的实现（spin包可以在Rust的create.io里找）
pub struct SpinLock<T>{
    locked: AtomicPtr<Cpu>,
    pub data: UnsafeCell<T>
}

unsafe impl<T> Sync for SpinLock<T> {}
unsafe impl<T> Send for SpinLock<T> {}

//上锁时会获得SpinLockGurd
//spinlock是锁的本体
//intr_lock是中断锁的守护结构体，当被解锁时会释放
pub struct SpinLockGurd<'a , T:'a>{
    spinlock: &'a SpinLock<T>,
    intr_lock: IntrLockGurd<'a>,
}

impl<T> SpinLock<T> {
    pub const fn new(data:T) -> Self {
        Self { 
            locked: AtomicPtr::new(ptr::null_mut()), 
            data: UnsafeCell::new(data), 
        }
    }

    pub fn holding(&self) -> bool {
        self.locked.load(Ordering::Relaxed) == CPUS.my_cpu()
    }
//此处的intr_lock是为了让程序在被锁期间不能被中断打断
//相信见process/cpu.rs
    pub fn locked(&self) -> SpinLockGurd<'_,T>{
        let intr_lock = CPUS.intr_lock();
        loop {
            if self.locked.compare_exchange_weak(
                ptr::null_mut(),
                CPUS.my_cpu(),
                Ordering::Acquire,
                Ordering::Relaxed
            ).is_err(){
                core::hint::spin_loop()
            }
            break SpinLockGurd { 
                spinlock: self,
                intr_lock, 
            };
        }
    }

    pub fn unlock(gurd : SpinLockGurd<'_,T>) -> &'_ SpinLock<T>{
        gurd.spinlock()
    }

    pub unsafe fn get_mut(&self) -> &mut T{
        &mut *self.data.get()
    }
}

impl <'a,T:'a> SpinLockGurd<'a,T>{
    pub fn spinlock(&self) -> &'a SpinLock<T>{
        self.spinlock
    }
    pub fn holding(&self) -> bool {
        self.spinlock.holding()
    }
}

impl<'a,T:'a> Drop for SpinLockGurd<'a,T>{
    fn drop(&mut self) {
        assert!(self.holding(), "release error");
        self.spinlock.locked.store(ptr::null_mut(),Ordering::Release);
    }
}

//为SpinLockGurd实现解引用
use core::ops::{Deref, DerefMut, Drop};

impl<'a, T: 'a> Deref for SpinLockGurd<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.spinlock.data.get() }
    }
}

impl<'a, T: 'a> DerefMut for SpinLockGurd<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.spinlock.data.get() }
    }
}

