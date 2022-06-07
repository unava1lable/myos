#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use core::panic::PanicInfo;
use bootloader::{ BootInfo, entry_point };
use myos::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use myos::memory;
    use myos::memory::BootInfoFrameAllocator;
    use x86_64::structures::paging::Size4KiB;
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello, World{}", "!");
    myos::init();

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let x = Box::new(41);
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    myos::hlt_loop();
}

// 非测试模式下的panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    myos::hlt_loop();
}

// 测试模式下的panic
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    myos::test_panic_handler(info);
}