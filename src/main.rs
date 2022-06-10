#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{ BootInfo, entry_point };
use myos::println;
use myos::task::keyboard;
use myos::task::{Task, simple_executor::SimpleExecutor};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;
    use myos::allocator;
    use myos::memory::{self, BootInfoFrameAllocator};

    println!("Hello World{}", "!");
    myos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
    
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

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}