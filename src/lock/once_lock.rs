use core::{sync::atomic::{Ordering,AtomicU32}, cell::{Cell, UnsafeCell}, mem::MaybeUninit, marker::PhantomData, fmt,};

//确保一个值的初始化只被初始化一次
///
/// # Examples
/// ```
/// use lock::once_lock::OnceLock;
/// static CELL:OnceLock<String> = Once::new();
/// 
/// let _ = CELL.set(||{
///     "Hello World".to_string();    
/// })
/// 
/// 
pub struct  OnceLock<T>{
    once: Once,
    value: UnsafeCell<MaybeUninit<T>>,
    _maker : PhantomData<T>, 
}

impl<T> OnceLock<T> {
    #[inline]
    pub const fn new() -> Self {
        Self { 
            once: Once::new(), 
            value: UnsafeCell::new(MaybeUninit::uninit()), 
            _maker: PhantomData, 
        }
    }
    #[inline]
    pub fn get(&self) -> Option<&T>{
        if self.is_initialize() {
            Some(unsafe {
                self.get_unchecked()
            })
        }else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&self) -> Option<&T>{
        if self.is_initialize(){
            Some(unsafe {
                self.get_unchecked_mut()
            })
        }else {
            None
        }
    }
    #[inline]
    pub fn set(&self,value:T) -> Result<(),T>{
        let mut value = Some(value);
        self.get_or_init(||{value.take().unwrap()});
        match value {
            None => Ok(()),
            Some(value) => Err(value),
        }
    }
    #[inline]
    pub fn get_or_init<F>(&self,f:F) -> &T
    where
        F:FnOnce()-> T
    {
        match self.get_or_try_init(|| Ok::<T,!>(f())) {
            Ok(val) => val,
            Err(e) => {e},
        }
    }
    #[inline]
    pub fn get_or_try_init<F,E>(&self,f:F) -> Result<&T,E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        if let Some(value) = self.get() {
            return Ok(value);
        }
        self.initialize(f)?;
        debug_assert!(self.is_initialize());
        
        Ok(unsafe {
            self.get_unchecked()
        })
    }
    #[inline]
    pub fn into_inner(mut self) -> Option<T>{
        self.take()
    }
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        if self.is_initialize() {
            self.once = Once::new();
            unsafe { Some((&mut *self.value.get()).assume_init_read()) }
        } else {
            None
        }
    }
    #[inline]
    fn is_initialize(&self) -> bool {
        self.once.is_completed()
    }
    #[cold]
    pub fn initialize<F,E>(&self,f:F) -> Result<(),E>
    where 
        F:FnOnce() -> Result<T,E>
    {
        let mut res:Result<(),E> = Ok(());
        let slot: &UnsafeCell<MaybeUninit<T>> = &self.value;
        // Ignore poisoning from other threads
        // If another thread panics, then we'll be able to run our closure
        self.once.call_once_force(|p|{
            match f() {
                Ok(value) => {
                    unsafe { (&mut *slot.get()).write(value) };
                }
                Err(e) => {
                    res = Err(e);
                    // Treat the underlying `Once` as poisoned since we
                    // failed to initialize our value. Calls
                    p.poison()
                }
            }
        });
        res
    }

    /// # Safety
    ///
    /// The value must be initialized
    #[inline]
    unsafe fn get_unchecked(&self) -> &T {
        debug_assert!(self.is_initialize());
        (&*self.value.get()).assume_init_ref()
    }
    #[inline]
    unsafe fn get_unchecked_mut(&self) -> &mut T {
        debug_assert!(self.is_initialize());
        (&mut *self.value.get()).assume_init_mut()
    }
}


// Why do we need `T: Send`?
// Thread A creates a `OnceLock` and shares it with
// scoped thread B, which fills the cell, which is
// then destroyed by A. That is, destructor observes
// a sent value.
unsafe impl<T: Sync + Send> Sync for OnceLock<T> {}
unsafe impl<T: Send> Send for OnceLock<T> {}

//impl<T: RefUnwindSafe + UnwindSafe> RefUnwindSafe for OnceLock<T> {}
//impl<T: UnwindSafe> UnwindSafe for OnceLock<T> {}

impl<T> Default for OnceLock<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T:fmt::Debug> fmt::Debug for OnceLock<T>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get() {
            Some(value) => f.debug_tuple("Once").field(value).finish(),
            None => f.write_str("Once(Uninit)")
        }
    }
}

impl<T:Clone> Clone for OnceLock<T> {
    fn clone(&self) -> Self {
        let cell = Self::new();
        if let Some(value) = self.get(){
            match cell.set(value.clone()) {
                Ok(()) => (),
                Err(_) => unreachable!()
            }
        }
        cell
    }
}
impl <T> From<T> for OnceLock<T> {
    fn from(value: T) -> Self {
        let cell = Self::new();
        match cell.set(value) {
            Ok(()) => cell,
            Err(_) => unreachable!()
        }
    }
}
impl<T: PartialEq> PartialEq for OnceLock<T> {
    #[inline]
    fn eq(&self, other: &OnceLock<T>) -> bool {
        self.get() == other.get()
    }
}

impl<T: Eq> Eq for OnceLock<T> {}

impl<T> Drop for OnceLock<T> {
    fn drop(&mut self) {
        if self.is_initialize(){
            unsafe { (&mut *self.value.get()).assume_init_drop() };
        }
    }
}


/// No initialization has run yet, and no thread is currently using the Once.
const INCOMPLETE: u32 = 0;
/// Some thread has previously attempted to initialize the Once, but it panicked,
/// so the Once is now poisoned. There are no other threads currently accessing
/// this Once.
const POISONED: u32 = 1;
/// Some thread is currently attempting to run initialization. It may succeed,
/// so all future threads need to wait for it to finish.
const RUNNING: u32 = 2;
/// Some thread is currently attempting to run initialization and there are threads
/// waiting for it to finish.
const QUEUED: u32 = 3;
/// Initialization has completed and all future calls should finish immediately.
const COMPLETE: u32 = 4;


///用来保证一个段代码就算在多线程的情况下也只执行一次
/// # Examples
/// ```
/// use lock::Once
/// static INIT:Once = Once::new();
/// fn foo(){
///     INIT.call_once(||{
///         //在这里执行初始化代码
///     })
/// }
/// 
/// ```
pub struct Once{
    state:AtomicU32,
}

pub struct OnceState{
    poisoned: bool,
    set_state_to:Cell<u32>,
}

struct OnceGurd<'a>{
    state:&'a AtomicU32,
    set_state_on_drop_to:u32,
}

impl OnceState {
    #[inline]
    pub fn is_poisoned(&self) -> bool{
        self.poisoned
    }
    #[inline]
    pub fn poison(&self){
        self.set_state_to.set(POISONED);
    }
}

impl Once{
    #[inline]
    pub const fn new() -> Self {
        Once { state: AtomicU32::new(INCOMPLETE) }
    }
    #[inline]
    pub fn is_completed(&self) -> bool {
        self.state.load(Ordering::Acquire) == COMPLETE
    }
    #[inline]
    pub fn call_once<F>(&self,f:F)
    where 
        F:FnOnce(), 
    {
        if self.is_completed(){
            return;
        }
        let mut f = Some(f);
        self.call(false, &mut |_| f.take().unwrap()());
    }

    pub fn call_once_force<F>(&self,f:F)
    where
        F:FnOnce(&OnceState)
    {
        if self.is_completed(){
            return;
        }
        let mut f = Some(f);
        self.call(true, &mut |p|f.take().unwrap()(p));
    }


    #[cold]
    fn call(&self,ignore_posioning:bool,f:&mut impl FnMut(&OnceState)){
        let mut state = self.state.load(Ordering::Acquire);
        loop{
            match state {
                POISONED if !ignore_posioning => {
                    panic!("Once instance has previously been poisoned");
                }
                INCOMPLETE | POISONED => {
                    if let Err(new) = 
                        self.state.compare_exchange_weak(
                            state, RUNNING, Ordering::Acquire, Ordering::Acquire)
                    {
                        state = new;
                        continue;
                    }
                    
                    let mut waiter_queue = 
                        OnceGurd{state:&self.state,set_state_on_drop_to:POISONED};
                    
                    let f_state = OnceState {
                        poisoned:state == POISONED,
                        set_state_to:Cell::new(COMPLETE),
                    };

                    f(&f_state);
                    waiter_queue.set_state_on_drop_to = f_state.set_state_to.get();
                    break;
                }
                RUNNING | QUEUED => {
                    core::hint::spin_loop();
                    state = self.state.load(Ordering::Acquire);
                    
                }
                COMPLETE => break,
                _ => unreachable!("state is never set to invalid values"),
            }
        }
    
    }
}
//当他被释放的时候，改变了state
//程序在spin_loop收到被改变的信号，然后跳出循环继续执行
impl<'a> Drop for OnceGurd<'a>{
    fn drop(&mut self) {
        let state = 
            self.state.swap(self.set_state_on_drop_to, Ordering::AcqRel);
    }
}