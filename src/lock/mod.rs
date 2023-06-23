//该模块是程序的Lock模块
//目前集成了一个锁：自旋锁（Spinlock

mod mutex;
mod spin;

pub use self::spin::SpinLock;
pub use self::spin::SpinLockGurd;