use core::{sync::atomic::AtomicBool, cell::UnsafeCell};
//FIXME #2(Who): 现目前的Spinlock有一个不知道是特性还是问题： 在xv6里面为什么需要指向当前的cpu结构体
pub struct Mutex<T: ?Sized>{
    locked: AtomicBool,
    data: UnsafeCell<T>
}


impl<T> Mutex<T> {
    
}