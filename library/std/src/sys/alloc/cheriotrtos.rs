//! Currently offloaded to the RTOS.

use crate::alloc::{GlobalAlloc, Layout, System};
use crate::sys::pal::abi;

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { abi::cheriot_alloc(layout.size() as _, layout.align() as _) as _ }
    }
    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            abi::cheriot_free(ptr as _);
        }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // RTOS allocations are zeroed
        unsafe { self.alloc(layout) }
    }
}
