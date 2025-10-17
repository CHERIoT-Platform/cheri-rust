## What is `libcheriot`
This library is a collection of utilities that can be used to run tests on CHERIoT platforms. 
It offers implementations of the panic handler, the global allocator and `print!` macros family.

To implement these functions, `libcheriot` relies on `extern "C"` functions that are imported from the testing environment, in particular: 
```rust
unsafe extern "C" {
    pub fn __rust_test_alloc(bytes: u32) -> *mut core::ffi::c_void;
    pub fn __rust_test_free(ptr: *mut core::ffi::c_void);
    pub fn __rust_test_print_str(v: *const core::ffi::c_char);
    pub fn __rust_test_eprint_str(v: *const core::ffi::c_char);
    pub fn __rust_test_panic(v: *const core::ffi::c_char) -> !;
}
```
