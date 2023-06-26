use core::{cell::Cell, ops::Deref};

use super::OnceLock;
//让一个值在第一次访问的时候被初始化，且只能初始化一次
//用了OnceLock实现
/// # Examples
/// ```
/// use lock::lazylock::LzayLock;
/// let x:LazyLock = LazyLock::new(||String::from("hello".to_string()));
/// let a = &(*x) 
///```
/// 解引用的时候会初始化
pub struct LazyLock<T,F = fn() -> T> {
    cell:OnceLock<T>,
    init:Cell<Option<F>>
}

unsafe impl<T, F: Send> Sync for LazyLock<T, F> where OnceLock<T>: Sync {}

impl<T,F> LazyLock<T,F> {
    pub fn new(f:F) -> Self{
        Self{
            cell:OnceLock::new(),
            init:Cell::new(Some(f)),
        }
    }

    pub fn force(this:&Self) -> &T
    where 
        F:FnOnce() -> T
    {
        this.cell.get_or_init(||match this.init.take() {
            Some(f) => f(),
            None => panic!("Lazy instance has previously been poisoned"),
        })
    }

    pub fn get(&self) -> Option<&T>{
        self.cell.get()
    }
}

impl<T, F: FnOnce() -> T> Deref for LazyLock<T, F> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        Self::force(self)
    }
}
impl<T: Default> Default for LazyLock<T, fn() -> T> {
    fn default() -> Self {
        Self::new(T::default)
    }
}