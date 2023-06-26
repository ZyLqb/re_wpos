use core::fmt::{self,Write,Arguments};
use core::sync::atomic::AtomicBool;
//use crate::lock::SpinLock;

use crate::sbi::consele_putchar;
use crate::lock::SpinLock; 
pub static PR :SpinLock<Writer> = SpinLock::new(Writer);
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
    PR.locked().write_fmt(args).expect("print error");
}