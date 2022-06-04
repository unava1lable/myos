#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use myos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, myos!");
    myos::init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

// 非测试模式下的panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

// 测试模式下的panic
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    myos::test_panic_handler(info);
}