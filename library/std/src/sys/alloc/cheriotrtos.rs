//! Currently offloaded to the RTOS.

use super::MIN_ALIGN;
use crate::alloc::Layout;
use crate::ptr;
use crate::sys::pal::abi;

#[inline]
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    if layout.align() > MIN_ALIGN {
        // we do not currently support overalignment
        return ptr::null_mut();
    }

    unsafe { abi::cheriot_alloc(layout.size() as u32) }
}
#[inline]
pub unsafe fn dealloc(ptr: *mut u8, _layout: Layout) {
    unsafe {
        abi::cheriot_free(ptr as _);
    }
}

#[inline]
pub unsafe fn alloc_zeroed(layout: Layout) -> *mut u8 {
    // RTOS allocations are zeroed
    unsafe { alloc(layout) }
}

#[inline]
pub unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    unsafe { super::realloc_fallback(ptr, layout, new_size) }
}
