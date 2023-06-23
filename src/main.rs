#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![allow(unused)]
#[macro_use]
mod console;
mod sbi;
mod process;
mod lock;
mod utiles;
mod riscv;
use core::arch::global_asm;

use crate::{lock::SpinLock, process::cpu::CPUS, sbi::r_tp};
//将entry.rs 加入代码
global_asm!(include_str!("entry.s"));
#[no_mangle]
fn rust_main() {
    //清空bss段
    utiles::clear_bss();
    //换新多线程
    sbi::thread_start();
    
    let thread_id = r_tp();
    if thread_id == 0 {
        println!("Thread {} start !!!",thread_id);
    }else {
        println!("Thread {} start !!!",thread_id);
    }
    panic!("Here is end");
}