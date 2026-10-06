//! Our allocator.

extern crate alloc;

unsafe extern "C" {
    pub fn cheriot_alloc(bytes: u32) -> *mut core::ffi::c_void;
    pub fn cheriot_free(ptr: *mut core::ffi::c_void);
}

/// An allocator based on the CHERIoT RTOS allocator.
struct CHERIoTRTOSAllocator;

unsafe impl alloc::alloc::GlobalAlloc for CHERIoTRTOSAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe { cheriot_alloc(layout.size() as _) as _ }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe { cheriot_free(ptr as _) }
    }
}

#[global_allocator]
static CHERIOT_RTOS_ALLOCATOR: CHERIoTRTOSAllocator = CHERIoTRTOSAllocator;
