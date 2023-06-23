    .section .text.entry
    .global _start
_start:
    add tp,a0,x0
    la sp,bootstack
    li t0,4096*2
    addi a0,a0,1
    mul a0,a0,t0
    add sp,sp,a0
    call rust_main
    .section .bss.stack
    .align 12
    .global bootstack
bootstack:
    .space 4096*6
    .global bootstacktop
bootstacktop: