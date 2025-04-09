//! Simple user-space application

#![no_std]
#![no_main]
#![feature(linkage)]

mod lang_items;
mod syscall;

pub use syscall::*;

/// Clear the BSS segment
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8). write_volatile(0)}
    });
}

#[no_mangle]
#[link_section = ".text.entry"]
/// The user program
pub extern "C" fn _start() {
    clear_bss();
    syscall::sys_exit(main());
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    let s = "Cannot find main!\n";
    syscall::sys_write(1, s.as_bytes());
    9
}