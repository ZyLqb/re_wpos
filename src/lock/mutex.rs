use core::{sync::atomic::AtomicBool, cell::UnsafeCell};

pub struct Mutex<T: ?Sized>{
    locked: AtomicBool,
    data: UnsafeCell<T>
}


impl<T> Mutex<T> {
    
}