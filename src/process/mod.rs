pub mod context;
use alloc::alloc::{alloc, dealloc, Layout};
use context::Context;

const STACK_SIZE: usize = 0x80000;

pub struct KernelStack(usize);

impl KernelStack {
	pub fn new() -> Self {
		let bottom = unsafe {
			alloc(Layout::from_size_align(STACK_SIZE, STACK_SIZE).unwrap())
		} as usize;
		Self(bottom)
	}
}

impl Drop for KernelStack {
	fn drop(&mut self) {
		unsafe {
			dealloc(self.0 as _, Layout::from_size_align(STACK_SIZE, STACK_SIZE).unwrap())
		};
	}
}

pub struct Thread {
	pub context: Context,
	pub kernel_stack: KernelStack,
}

impl Thread {
	pub fn switch_to(&mut self, next: &mut Context) {
		unsafe {
			context::switch_to(&mut self.context, next)
		};
	}
}