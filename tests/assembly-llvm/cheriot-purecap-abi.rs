//@ assembly-output: emit-asm
//@ only-riscv32cheriot-unknown-cheriotrtos

// Ensure we emit the correct calling conventions in specific important cases.

#![no_std]
#![crate_type = "lib"]

// Little mischievous helper to avoid transmuting i64 into pointers,
// which makes LLVM sit down and cry.
#[inline(always)]
fn ptr_helper(s: i64) -> *const () {
    unsafe {
        union X {
            int: i64,
            ptr: *const (),
        }

        X { int: s }.ptr
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn int(s: i64) -> i64 {
    // Uses two registers, the integer is split in two (a0 and a1).

    // CHECK: int:
    // CHECK:       addi   a0, a0, 1
    // CHECK:       seqz   a2, a0
    // CHECK:       add       a1, a1, a2
    // CHECK:       addi   a1, a1, 1
    // CHECK:       ct.cret

    s + (1 << 32) + 1
}

// On CHERI systems, this int struct may contain a hidden pointer, and we don't
// want to lose its tag: it must be passed as a pointer so that it doesn't get
// split in two registers.
#[repr(C)]
pub struct IntStruct {
    int: i64,
}

#[unsafe(no_mangle)]
pub extern "C" fn int_struct(s: IntStruct) -> *const () {
    // Not the best in term of codegen, but a0 contains our integer value,
    // it's stored on the stack and then re-read in two tranches into an invalid capability.
    // The important thing to check here is that `s` is passed in a single register.

    // CHECK: int_struct:
    // CHECK:    ct.cincoffset   sp, sp, -16
    // CHECK:    ct.csc   a0, 8(sp)
    // CHECK:    ct.clw   a0, 12(sp)
    // CHECK:    ct.clw   a1, 8(sp)
    // CHECK:    ct.csethigh   a0, a1, a0
    // CHECK:    ct.cincoffset   sp, sp, 16
    // CHECK:    ct.cret

    unsafe { ptr_helper(s.int) }
}

// Similarly, the ABI mandates that aggregates up to 2*YLEN must be passed in capability
// registers if their alignment is at least YLEN.
#[repr(C)]
pub struct TwoIntsStruct {
    int1: i64,
    int2: i64,
}

#[repr(C)]
pub struct TwoPtrStruct {
    ptr1: *const (),
    ptr2: *const (),
}

#[unsafe(no_mangle)]
pub extern "C" fn two_ints_struct(s: TwoIntsStruct) -> TwoPtrStruct {
    // Same as `int_struct`, `s` here must be passed in two registers (a0 and a1).

    // CHECK: two_ints_struct:
    // CHECK:     ct.cincoffset   sp, sp, -16
    // CHECK:     ct.csc   a0, 8(sp)
    // CHECK:     ct.csc   a1, 0(sp)
    // CHECK:     ct.clw   a0, 12(sp)
    // CHECK:     ct.clw   a1, 8(sp)
    // CHECK:     ct.clw   a2, 4(sp)
    // CHECK:     ct.clw   a3, 0(sp)
    // CHECK:     ct.csethigh   a0, a1, a0
    // CHECK:     ct.csethigh   a1, a3, a2
    // CHECK:     ct.cincoffset   sp, sp, 16
    // CHECK:     ct.cret

    TwoPtrStruct { ptr1: unsafe { ptr_helper(s.int1) }, ptr2: unsafe { ptr_helper(s.int2) } }
}

#[unsafe(no_mangle)]
pub extern "C" fn a_ptr(ptr: *const i8) -> i8 {
    // An easy one, just a capability in a single register.

    // CHECK: a_ptr:
    // CHECK:   ct.clb   a0, 0(a0)
    // CHECK:   ct.cret

    unsafe { *ptr }
}

#[repr(C)]
pub struct PtrStruct {
    ptr: *const i64,
}

#[unsafe(no_mangle)]
pub extern "C" fn ptr_struct(s: PtrStruct) -> i64 {
    // Another easy one, we load the capability in `a0` and read the memory.

    // CHECK:  ptr_struct:
    // CHECK:       ct.clw   a2, 0(a0)
    // CHECK:       ct.clw   a1, 4(a0)
    // CHECK:       mv   a0, a2
    // CHECK:       ct.cret

    unsafe { *s.ptr }
}

#[repr(C)]
pub struct IntAndPtrStruct {
    int1: i64,
    ptr: *const (),
}

#[unsafe(no_mangle)]
pub extern "C" fn int_and_ptr_struct(s: IntAndPtrStruct) -> TwoPtrStruct {
    // Must be passed in two registers and returned in two registers.
    // Capability-sized register a0 contains the integer, and a1 contains the capability.
    // Same ugly trick to avoid `inttoptr`s.

    // CHECK: int_and_ptr_struct:
    // CHECK:     ct.cincoffset   sp, sp, -16
    // CHECK:     ct.csc   a0, 8(sp)
    // CHECK:     ct.clw   a0, 12(sp)
    // CHECK:     ct.clw   a2, 8(sp)
    // CHECK:     ct.csethigh   a0, a2, a0
    // CHECK:     ct.cincoffset   sp, sp, 16
    // CHECK:     ct.cret

    TwoPtrStruct { ptr1: unsafe { ptr_helper(s.int1) }, ptr2: s.ptr }
}

#[unsafe(no_mangle)]
pub extern "C" fn float(s: f64) -> f64 {
    // `s` should be passed in a single capability-wide register.

    // CHECK: float:
    // CHECK:     ct.cincoffset   sp, sp, -16
    // CHECK:     ct.csc   ra, 8(sp)
    // CHECK:     lui   a1, 548864
    // CHECK:     lui   a2, 268901
    // CHECK:     addi   a2, a2, 530
    // CHECK:     ct.csethigh   a1, a1, a2
    // CHECK: .LBB6_1:
    // CHECK:     ct.auipcc   t2, %cheriot_compartment_code_hi(__library_import_libcalls___adddf3)
    // CHECK:     ct.clc   t2, %cheriot_compartment_lo_i(.LBB6_1)(t2)
    // CHECK:     ct.cjalr   t2
    // CHECK:     ct.clc   ra, 8(sp)
    // CHECK:     ct.cincoffset   sp, sp, 16
    // CHECK:     ct.cret

    s + 187238723.0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn variadic(count: u32, mut ap: ...) -> i32 {
    // Almost correct: the variadic arguments are indeed passed on the stack,
    // but the capability to them is not passed in register x5 as mandated by the ABI.
    // It appears to be a choice of LLVM, though, not Rust.

    // CHECK: variadic:
    // CHECK:    ct.cincoffset   sp, sp, -16
    // CHECK:    mv   a1, a0
    // CHECK:    ct.cincoffset   a0, sp, 8
    // CHECK:    ct.csetbounds   a0, a0, 8
    // CHECK:    ct.cincoffset   a2, sp, 16
    // CHECK:    ct.csc   a2, 0(a0)
    // CHECK:    li   a0, 0
    // CHECK:    beqz   a1, .LBB7_3
    // CHECK:    ct.clc   a2, 8(sp)
    // CHECK:    slli   a1, a1, 2
    // CHECK:    ct.cincoffset   a1, a2, a1
    // CHECK: .LBB7_2:
    // CHECK:    ct.cmove   a3, a2
    // CHECK:    ct.cincoffset   a2, a2, 4
    // CHECK:    ct.csc   a2, 8(sp)
    // CHECK:    ct.clw   a3, 0(a3)
    // CHECK:    add   a0, a0, a3
    // CHECK:    bne   a2, a1, .LBB7_2
    // CHECK: .LBB7_3:
    // CHECK:    ct.cincoffset   sp, sp, 16
    // CHECK:    ct.cret

    let mut sum = 0;
    for _ in 0..count {
        sum += unsafe { ap.next_arg::<i32>() };
    }
    sum
}

#[unsafe(no_mangle)]
unsafe extern "C" fn smaller_int(s: i32) -> i32 {
    // Pretty simple: a0 contains the i32.

    // CHECK: smaller_int:
    // CHECK:    addi   a0, a0, 1
    // CHECK:    ct.cret

    s + 1
}

// Should be passed in two registers.
#[repr(C)]
pub struct PtrAndSmallerIntStruct {
    ptr: *const (),
    int: i32,
}

#[unsafe(no_mangle)]
pub extern "C" fn ptr_and_smaller_int_struct(s: PtrAndSmallerIntStruct) -> TwoPtrStruct {
    // Register a0 should contain the pointer, a1 should contain the i32.

    // CHECK: ptr_and_smaller_int_struct:
    // CHECK:    ct.cincoffset   sp, sp, -16
    // CHECK:    ct.csc   a1, 8(sp)
    // CHECK:    ct.clw   a1, 8(sp)
    // CHECK:    srai   a2, a1, 31
    // CHECK:    ct.csethigh   a1, a1, a2
    // CHECK:    ct.cmove   a2, a0
    // CHECK:    ct.cmove   a0, a1
    // CHECK:    ct.cmove   a1, a2
    // CHECK:    ct.cincoffset   sp, sp, 16
    // CHECK:    ct.cret

    TwoPtrStruct { ptr1: ptr_helper(s.int as i64), ptr2: s.ptr }
}

// Should be spilled on the stack.
#[repr(C)]
pub struct PtrAndTwoIntsStruct {
    ptr: *const (),
    int1: i64,
    int2: i64,
}

#[unsafe(no_mangle)]
pub extern "C" fn ptr_and_two_ints_struct(s: PtrAndTwoIntsStruct) -> TwoPtrStruct {
    // All the values should be loaded indirectly from a0.

    // CHECK: ptr_and_two_ints_struct:
    // CHECK:    ct.clw   a1, 8(a0)
    // CHECK:    ct.clw   a2, 12(a0)
    // CHECK:    ct.clw   a3, 20(a0)
    // CHECK:    ct.clw   a4, 16(a0)
    // CHECK:    add   a2, a2, a3
    // CHECK:    add   a3, a4, a1
    // CHECK:    ct.clc   a1, 0(a0)
    // CHECK:    sltu   a0, a3, a4
    // CHECK:    add   a0, a0, a2
    // CHECK:    ct.csethigh   a0, a3, a0
    // CHECK:    ct.cret

    TwoPtrStruct { ptr1: ptr_helper(s.int1 + s.int2), ptr2: s.ptr }
}
