#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![allow(unused)]
#![feature(never_type)]

#[macro_use]
mod console;
mod sbi;
mod process;
mod lock;
mod utiles;
mod riscv;
mod kalloc;
mod vm;
mod memlayout;
#[macro_use]
extern crate alloc;

use core::{arch::global_asm, sync::atomic::{AtomicBool,Ordering}};
use crate::kalloc::pagealloc;
use sbi::r_tp;
static STARTED: AtomicBool = AtomicBool::new(false);
//将entry.rs 加入代码
global_asm!(include_str!("entry.s"));
#[no_mangle]
fn rust_main() {
    //开启多线程
    sbi::thread_start();
    let thread_id = r_tp();
    if thread_id == 0 {
        //heap init 
        kalloc::init();
        //page alloc init
        pagealloc::init();
        //kvm init
        vm::k_init();
        //install pagetable
        vm::kvminithart();

        println!("Thread {} start !!!",thread_id);
        STARTED.store(true, Ordering::SeqCst);
    }else {
        loop {if STARTED.load(Ordering::SeqCst){break;}}
        vm::kvminithart();
        println!("Thread {} start !!!",thread_id);
    }
    loop {}
}