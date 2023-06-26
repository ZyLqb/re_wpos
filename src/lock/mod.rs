//该模块是程序的Lock模块
//目前集成了一个锁：自旋锁（Spinlock
//TODO #1: 实现RwLock   --complated(L19ht)!!!
//TODO #2: 实现OnceLock --complated(L19ht)!!!
//TODO #3: 互斥锁的实现 
//TODO #4: 实现LazyLock --complated(L19ht)!!!
//TODO #5: 实现
mod mutex;
mod spin;
mod once_lock;
mod lazylock;
mod rwlock;

pub use self::once_lock::OnceLock;
pub use self::once_lock::Once;
pub use self::once_lock::OnceState;

pub use self::lazylock::LazyLock;

pub use self::rwlock::RwLock;
pub use self::rwlock::ReadGurd;
pub use self::rwlock::WriteGurd;

pub use self::spin::SpinLock;
pub use self::spin::SpinLockGurd;