//@ only-riscv32cheriot-unknown-cheriotrtos

// Ensure we emit the correct calling conventions in specific important cases.

#![no_std]
#![crate_type = "lib"]

// CHECK: define dso_local noundef range(i64 0, -8589934591) i64 @int(i64 noundef %s)
#[unsafe(no_mangle)]
pub extern "C" fn int(s: i64) -> i64 {
    s + 1 << 32 + 1
}

// On CHERI systems, this int struct may contain a hidden pointer, and we don't
// want to lose its tag: it must be passed as a pointer so that it doesn't get
// split in two registers.
#[repr(C)]
pub struct IntStruct {
    int: i64,
}

// CHECK: define dso_local noundef ptr addrspace(200) @int_struct(ptr addrspace(200) nofree readnone returned captures(ret: address, provenance) %0)
#[unsafe(no_mangle)]
pub extern "C" fn int_struct(s: IntStruct) -> *const () {
    unsafe { core::mem::transmute(s) }
}

// Similarly, the ABI mandates that aggregates up to 2*YLEN must be passed in
// capability registers if their alignment is at least YLEN.
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

// CHECK: define dso_local { ptr addrspace(200), ptr addrspace(200) } @two_ints_struct({ ptr addrspace(200), ptr addrspace(200) } %0)
#[unsafe(no_mangle)]
pub extern "C" fn two_ints_struct(s: TwoIntsStruct) -> TwoPtrStruct {
    TwoPtrStruct {
        ptr1: unsafe { core::mem::transmute(s.int1) },
        ptr2: unsafe { core::mem::transmute(s.int2) },
    }
}

// CHECK: define dso_local noundef signext i8 @a_ptr(ptr addrspace(200) nofree noundef readonly captures(none) %ptr)
#[unsafe(no_mangle)]
pub extern "C" fn a_ptr(ptr: *const i8) -> i8 {
    unsafe { *ptr }
}

#[repr(C)]
pub struct PtrStruct {
    ptr: *const i64,
}

// CHECK: define dso_local noundef i64 @ptr_struct(ptr addrspace(200) nofree readonly captures(none) %0)
#[unsafe(no_mangle)]
pub extern "C" fn ptr_struct(s: PtrStruct) -> i64 {
    unsafe { *s.ptr }
}

#[repr(C)]
pub struct IntAndPtrStruct {
    int: i64,
    ptr: *const (),
}

// CHECK: define dso_local { ptr addrspace(200), ptr addrspace(200) } @int_and_ptr_struct({ ptr addrspace(200), ptr addrspace(200) } %0)
#[unsafe(no_mangle)]
pub extern "C" fn int_and_ptr_struct(s: IntAndPtrStruct) -> TwoPtrStruct {
    TwoPtrStruct { ptr1: unsafe { core::mem::transmute(s.int) }, ptr2: s.ptr }
}

// CHECK: define dso_local noundef double @float(double noundef %s) unnamed_addr addrspace(200)
#[unsafe(no_mangle)]
pub extern "C" fn float(s: f64) -> f64 {
    s + 187238723.0
}

// CHECK: define dso_local noundef i32 @variadic(i32 noundef %count, ...) unnamed_addr addrspace(200)
#[unsafe(no_mangle)]
unsafe extern "C" fn variadic(count: u32, mut ap: ...) -> i32 {
    let mut sum = 0;
    for _ in 0..count {
        sum += unsafe { ap.next_arg::<i32>() };
    }
    sum
}

// CHECK: define dso_local noundef i32 @smaller_int(i32 noundef %s)
#[unsafe(no_mangle)]
unsafe extern "C" fn smaller_int(s: i32) -> i32 {
    s + 1
}

// Should be passed in two registers.
#[repr(C)]
struct PtrAndSmallerIntStruct {
    ptr: *const (),
    int: i32,
}

// CHECK: define dso_local { ptr addrspace(200), ptr addrspace(200) } @ptr_and_smaller_int_struct({ ptr addrspace(200), ptr addrspace(200) } %0)
#[unsafe(no_mangle)]
pub extern "C" fn ptr_and_smaller_int_struct(s: PtrAndSmallerIntStruct) -> TwoPtrStruct {
    TwoPtrStruct { ptr1: unsafe { core::mem::transmute(s.int as i64) }, ptr2: s.ptr }
}

// Should be spilled on the stack.
#[repr(C)]
struct PtrAndTwoIntsStruct {
    ptr: *const (),
    int1: i64,
    int2: i64,
}

// CHECK: define dso_local { ptr addrspace(200), ptr addrspace(200) } @ptr_and_two_ints_struct(ptr addrspace(200) noalias nofree noundef readonly align 8 captures(none) dead_on_return dereferenceable(24) %s)
#[unsafe(no_mangle)]
pub extern "C" fn ptr_and_two_ints_struct(s: PtrAndTwoIntsStruct) -> TwoPtrStruct {
    TwoPtrStruct { ptr1: unsafe { core::mem::transmute(s.int1 + s.int2) }, ptr2: s.ptr }
}
