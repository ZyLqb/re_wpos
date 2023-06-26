use core::{sync::atomic::{AtomicPtr, AtomicUsize, AtomicBool,Ordering}, cell::UnsafeCell, hint::spin_loop, ops::{Deref, DerefMut}};

use crate::process::Cpu;

pub struct RwLock<T>{
    reader: AtomicUsize,
    writer: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

pub struct ReadGurd<'a ,T:'a> {
    lock: &'a RwLock<T>,
}
unsafe impl<'a,T: Send+'a> Send for ReadGurd<'a,T> {}
unsafe impl<'a,T: Send + Sync +'a> Sync for ReadGurd<'a,T> {}


pub struct WriteGurd<'a,T:'a> {
    lock: &'a RwLock<T>,
}

unsafe impl<'a,T: Send+'a> Send for WriteGurd<'a,T> {}
unsafe impl<'a,T: Send + Sync +'a> Sync for WriteGurd<'a,T> {}

impl<T> RwLock<T> {
    pub const fn new(data:T) -> Self{
        Self { 
            reader: AtomicUsize::new(0), 
            writer: AtomicBool::new(false), 
            data: UnsafeCell::new(data) 
        }
    }

    pub fn read(&self) -> ReadGurd<'_, T>{
        loop {
            match self.try_read(){
                Some(gurd) => break gurd,
                None => spin_loop(),
            }
        }
    }

    pub fn write(&self) -> WriteGurd<'_,T>{
        loop{
            match self.try_write() {
                Some(gurd) => break gurd,
                None => spin_loop(),
            }
        }
    }

    pub fn try_read(&self) -> Option<ReadGurd<T>>{        
        let value = 
            self.reader.fetch_add(1, Ordering::Acquire);
        if self.writer.load(Ordering::Relaxed) {
            self.reader.fetch_sub(1, Ordering::Release);
            None
        }else {
            Some({
                ReadGurd { lock: self }
            })
        }
    }

    pub fn try_write(&self) -> Option<WriteGurd<T>>{
        if self.writer.compare_exchange(
            false, 
            true, 
            Ordering::Acquire, 
            Ordering::Relaxed
        ).is_ok() && self.reader.load(Ordering::Relaxed) == 0 {
            Some(WriteGurd{lock:self})
        }else {
            None
        }
    }
}

impl<'a,T:'a> ReadGurd<'a,T>{
    pub fn upgrade(mut self) -> Self{
        loop {
            self = match self.try_upgrade() {
                Ok(gurd) => break gurd,
                Err(e) => e,
            };
            spin_loop()
        }
    }
    pub fn try_upgrade(self) -> Result<Self,Self>{
        if self.lock.writer.compare_exchange(
            false, 
            true, 
            Ordering::Acquire, 
            Ordering::Relaxed
        ).is_ok() && self.lock.reader.load(Ordering::Relaxed) == 1 {
            Ok(Self { lock:self.lock })
        }else {
            Err(self)
        }
    }
}

impl<'a,T:'a> Drop for ReadGurd<'a,T> {
    fn drop(&mut self) {
        assert!(self.lock.reader.load(Ordering::Relaxed) >= 1);
        self.lock.reader.fetch_sub(1,Ordering::Release);
    }
}

impl<'a,T:'a> Drop for WriteGurd<'a,T> {
    fn drop(&mut self) {
        assert!(self.lock.writer.load(Ordering::Relaxed));
        self.lock.writer.store(false,Ordering::Release);
    }
}

impl<'a,T:'a> Deref for ReadGurd<'a,T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a,T:'a> Deref for WriteGurd<'a,T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}
impl<'a,T:'a> DerefMut for WriteGurd<'a,T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}
