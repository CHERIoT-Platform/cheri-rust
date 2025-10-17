//! A bare-bones PRNG.

extern "C" {
    pub fn __rust_test_rand_int() -> core::ffi::c_int;
}

pub fn rand() -> i32 {
    unsafe { __rust_test_rand_int() }
}
