#![no_std]
#![feature(core_float_math)]

extern crate alloc;
extern crate cheriot;

// `f32::sqrt` emits a call to the `llvm.sqrt.32` intrinsic, which LLVM
// lowers to a libcall to `__library_export_libcalls_sqrtf`. The RTOS
// currently provides no such library function.
//
// A Rust implementation for this function is provided in `compiler-builtins/libm`,
// and defined for our target with the appropriate calling conventions. Previously,
// we did not build the `compiler-builtins` module, meaning this test would fail
// linking due to the missing symbol.
#[no_mangle]
extern "C" fn test_compiler_builtins() -> i32 {
    // Without `black_box` everything is optimised away.
    let positive = core::hint::black_box(4.0_f32);
    let negative = core::hint::black_box(-4.0_f32);
    let negative_zero = core::hint::black_box(-0.0_f32);

    assert_eq!(core::f32::math::sqrt(positive), 2.0);
    assert!(core::f32::math::sqrt(negative).is_nan());
    assert_eq!(core::f32::math::sqrt(negative_zero), negative_zero);

    0
}
