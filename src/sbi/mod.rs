#![allow(unused)]
use core::{arch::asm, usize, primitive};
const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;
const SBI_EXT_HSM :usize =0x48534D;


#[inline(always)]
fn sbi_call(sbi_type: usize, arg0: usize, arg1: usize, arg2: usize , arg3:usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x16") arg3,
            in("x17") sbi_type,
        );
    }
    ret
}

pub fn consele_putchar(c:usize){
    sbi_call(SBI_CONSOLE_PUTCHAR,c  ,0,0,0);
}
//system shutdown
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0,0);
    panic!("It should shutdown!");
}
pub fn sbi_hsm_hart_start(hart_id:usize){
    let i= sbi_call(SBI_EXT_HSM, hart_id, 0x80200000, 64, 0x0);
}
pub fn set_timer(time_value:usize){
    sbi_call(SBI_SET_TIMER,time_value ,0,0,0);
    //println!("sbi call {}", i);
}
// pub fn sbi_console_get_char() -> usize {
//     #[allow(deprecated)]
//     let ret = sbi_rt::legacy::console_getchar();
//     //info!("sbi.rs: ret{:#x}",ret);
//     ret
// }

#[inline]
pub fn r_tp() -> usize{
    unsafe{
        let id;
        asm!("mv {0}, tp", out(reg) id);
        id
    }
}
use crate::process::cpu::NCPU;
pub fn thread_start(){
    let tp = r_tp();
    let i:usize = 0;
    for i in i..NCPU {
        if i != tp {
            sbi_hsm_hart_start(i);
        }
    }
}