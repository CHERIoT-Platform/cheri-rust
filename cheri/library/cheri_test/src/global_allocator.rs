/// An allocator based on the CHERIoT RTOS allocator.
struct CHERIoTRTOSAllocator;

use core::alloc::Layout;
use core::ptr::null_mut;
use core::{cmp, ptr};

unsafe extern "C" {
    fn cheriot_alloc(bytes: u32) -> *mut core::ffi::c_void;
    fn cheriot_free(ptr: *mut core::ffi::c_void);
    fn is_valid(ptr: *mut core::ffi::c_void) -> bool;
}

unsafe impl alloc::alloc::GlobalAlloc for CHERIoTRTOSAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe {
            let ptr = cheriot_alloc(layout.size() as _);
            if !is_valid(ptr) {
                return null_mut();
            }
            return ptr as _;
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe { cheriot_free(ptr as _) }
    }

    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        unsafe { self.alloc(layout) }
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        // SAFETY: the caller must ensure that the `new_size` does not overflow.
        // `layout.align()` comes from a `Layout` and is thus guaranteed to be valid.
        let new_layout = unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };
        // SAFETY: the caller must ensure that `new_layout` is greater than zero.
        let new_ptr = unsafe { self.alloc(new_layout) };
        if !new_ptr.is_null() {
            // SAFETY: the previously allocated block cannot overlap the newly allocated block.
            // The safety contract for `dealloc` must be upheld by the caller.
            unsafe {
                ptr::copy_nonoverlapping(ptr, new_ptr, cmp::min(layout.size(), new_size));
                self.dealloc(ptr, layout);
            }
        }
        new_ptr
    }
}

#[global_allocator]
static CHERIOT_RTOS_ALLOCATOR: CHERIoTRTOSAllocator = CHERIoTRTOSAllocator;
