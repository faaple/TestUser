//! Simple user-space application

#![no_std]
#![no_main]

mod lang_items;
mod syscall;

/// Clear the BSS segment
fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8). write_volatile(0)}
    });
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
/// The user program
pub extern "C" fn _start() {
    clear_bss();
    let s = "Hello, world!\n";
    crate::syscall::sys_write(1, s.as_bytes());
    crate::syscall::sys_exit(9);
}