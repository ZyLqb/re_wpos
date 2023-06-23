use core::fmt::{self,Write,Arguments};
use core::sync::atomic::AtomicBool;
use spin::Mutex;

use crate::sbi::consele_putchar;
use crate::lock::SpinLock;
// FIXME#1(Who)：当一个线程panic之后应该释放PR锁,
// 让其他线程可以打印，这个应该可以直接在锁里面解决。 
pub static PR :Mutex<Writer> = Mutex::new(Writer);
pub struct Writer;

impl Write for Writer {
    fn write_str(&mut self,s:&str) -> fmt::Result{
        for c in s.chars(){
            consele_putchar(c as usize);
        }
        Ok(())
    }
}

#[allow(unused)]
pub fn print(args: Arguments) {
    PR.lock().write_fmt(args).expect("print error");
}