#![no_std]
#![no_main]

use core::panic::PanicInfo;
use myos::{ QemuExitCode, exit_qemu, serial_println, serial_print };

#[no_mangle]
pub extern "C" fn _start() -> ! {
	should_fail();
	serial_println!("[test did not panic]");
	exit_qemu(QemuExitCode::Failed);
	loop{}
}

fn should_fail() {
	serial_print!("should_fail... ");
	assert_eq!(1, 0);   // 注释该行会导致测试失败
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	serial_println!("[ok]");
	exit_qemu(QemuExitCode::Success);
	loop {}
}