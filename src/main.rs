#![no_std]
#![no_main]
mod vga;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("abcdefghijklmnopqrstuvwxyz{}", "!");
    panic!("Some panic message");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}