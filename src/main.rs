#![no_std]
#![no_main]
mod vga;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga::WRITER.lock(), ", some number: {} {}", 42, 1.337).unwrap();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}