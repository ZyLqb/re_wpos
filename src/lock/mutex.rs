use core::{sync::atomic::AtomicBool, cell::UnsafeCell};
//FIXME #2(Who):展示用的SpinLock代替的Mutex,也可以用SpinLock实现Mutex
pub struct Mutex<T: ?Sized>{
    locked: AtomicBool,
    data: UnsafeCell<T>
}


impl<T> Mutex<T> {
    
}