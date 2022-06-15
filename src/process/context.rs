use memoffset::offset_of;

#[repr(C)]
pub struct Context {
	cr3: usize,
	rbp: usize,
	rsp: usize,
	rbx: usize,
	r12: usize,
	r13: usize,
	r14: usize,
	r15: usize,
}

#[naked]
pub unsafe extern "C" fn switch_to(_prev: &mut Context, _next: &mut Context) {
	core::arch::asm!(
		// concat!("
		// mov rcx, cr3
		// mov [rdi + {cr3_offset}], rcx
		// mov rax, [rsi + {cr3_offset}]

		// mov [rdi + {rbp_offset}], rbp
		// mov rbp, [rsi + {rbp_offset}]

		// mov [rdi + {rsp_offset}], rsp
		// mov rsp, [rsi + {rsp_offset}]

		// mov [rdi + {rbx_offset}], rbx
		// mov rbx, [rsi + {rbx_offset}]

		// mov [rdi + {r12_offset}], r12
		// mov r12, [rsi + {r12_offset}]

		// mov [rdi + {r13_offset}], r13
		// mov r13, [rsi + {r13_offset}]

		// mov [rdi + {r14_offset}], r14
		// mov r14, [rsi + {r14_offset}]

		// mov [rdi + {r15_offset}], r15
		// mov r15, [rsi + {r15_offset}]
		// "),
		// cr3_offset = const(offset_of!(Context, cr3)),
		// rbp_offset = const(offset_of!(Context, rbp)),
		// rsp_offset = const(offset_of!(Context, rsp)),
		// rbx_offset = const(offset_of!(Context, rbx)),
		// r12_offset = const(offset_of!(Context, r12)),
		// r13_offset = const(offset_of!(Context, r13)),
		// r14_offset = const(offset_of!(Context, r14)),
		// r15_offset = const(offset_of!(Context, r15)),
		// options(noreturn),
		concat!("
		mov rcx, cr3
		mov [rdi + {cr3_offset}], rcx
		mov rax, [rsi + {cr3_offset}]

		mov [rdi + {rbp_offset}], rbp
		mov rbp, [rsi + {rbp_offset}]

		mov [rdi + {rsp_offset}], rsp
		mov rsp, [rsi + {rsp_offset}]

		mov [rdi + {rbx_offset}], rbx
		mov rbx, [rsi + {rbx_offset}]

		mov [rdi + {r12_offset}], r12
		mov r12, [rsi + {r12_offset}]

		mov [rdi + {r13_offset}], r13
		mov r13, [rsi + {r13_offset}]

		mov [rdi + {r14_offset}], r14
		mov r14, [rsi + {r14_offset}]

		mov [rdi + {r15_offset}], r15
		mov r15, [rsi + {r15_offset}]
		"),
		cr3_offset = const(0),
		rbp_offset = const(8),
		rsp_offset = const(16),
		rbx_offset = const(24),
		r12_offset = const(32),
		r13_offset = const(40),
		r14_offset = const(48),
		r15_offset = const(56),
		options(noreturn)
	)
}