#![no_std]
#![no_main]

/// 正确输出：
/// Hello world from user mode program!

#[no_mangle]
fn main() -> i32 {
    let s = "Hello World!\n";
    user_lib::sys_write(1, s.as_bytes());
    0
}