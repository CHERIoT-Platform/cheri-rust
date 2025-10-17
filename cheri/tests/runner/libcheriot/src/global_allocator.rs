/// An allocator based on the CHERIoT RTOS allocator.
struct _RustTestGlobalAllocator;

unsafe extern "C" {
    pub fn __rust_test_alloc(bytes: u32) -> *mut core::ffi::c_void;
    pub fn __rust_test_free(ptr: *mut core::ffi::c_void);
}

unsafe impl alloc::alloc::GlobalAlloc for _RustTestGlobalAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe { __rust_test_alloc(layout.size() as _) as _ }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe { __rust_test_free(ptr as _) }
    }
}

#[global_allocator]
static _RUST_TEST_GLOBAL_ALLOCATOR: _RustTestGlobalAllocator = _RustTestGlobalAllocator;
