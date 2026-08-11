//! Currently offloaded to the RTOS.
//!
//! The alignment padding code including comments comes from `library/std/src/sys/alloc/windows.rs`
//! mostly unchanged and needs review.

use super::MIN_ALIGN;
use crate::alloc::{GlobalAlloc, Layout, System};
use crate::ptr;
use crate::sys::pal::abi;

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= MIN_ALIGN {
            let ptr = unsafe { abi::cheriot_alloc(layout.size() as u32) as *mut u8 };
            return ptr;
        }

        // Allocate extra padding in order to be able to satisfy the alignment.
        // This addition does not overflow due to `Layout` type invariants,
        // `size()` is at most `isize::MAX` while
        // `align()` is at most `1 << (bits in usize - 2)` if `size()` is non-zero.
        let total = layout.align() + layout.size();

        let ptr = unsafe { abi::cheriot_alloc(total as u32) as *mut u8 };
        if ptr.is_null() {
            // Allocation has failed.
            return ptr::null_mut();
        }

        // Create a correctly aligned pointer offset from the start of the allocated block,
        // and write a header before it.

        let offset = layout.align() - (ptr.addr() & (layout.align() - 1));
        // SAFETY: `MIN_ALIGN` <= `offset` <= `layout.align()` and the size of the allocated
        // block is `layout.align() + layout.size()`. `aligned` will thus be a correctly aligned
        // pointer inside the allocated block with at least `layout.size()` bytes after it and at
        // least `MIN_ALIGN` bytes of padding before it.
        let aligned = unsafe { ptr.add(offset) };
        // SAFETY: Because the size and alignment of a header is <= `MIN_ALIGN` and `aligned`
        // is aligned to at least `MIN_ALIGN` and has at least `MIN_ALIGN` bytes of padding before
        // it, it is safe to write a header directly before it.
        unsafe { ptr::write((aligned as *mut *mut u8).sub(1), ptr) };

        // SAFETY: The returned pointer does not point to the start of an allocated block,
        // but there is a header readable directly before it containing the location of the start
        // of the block.
        aligned
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let block = if layout.align() <= MIN_ALIGN {
            ptr
        } else {
            // The location of the start of the block is stored in the padding before `ptr`.

            // SAFETY: Because of the contract of `System`, `ptr` is guaranteed to be non-null
            // and have a header readable directly before it.
            unsafe { ptr::read((ptr as *mut *mut u8).sub(1)) }
        };

        unsafe { abi::cheriot_free(block as _) };
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // `heap_allocate` is always zeroed
        unsafe { self.alloc(layout) }
    }

    // This is super::relloc_fallback, copied here as the current implementation is CHERI safe
    // but may not be in the future
    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
        unsafe {
            let new_layout = Layout::from_size_align_unchecked(new_size, old_layout.align());

            let new_ptr = GlobalAlloc::alloc(self, new_layout);
            if !new_ptr.is_null() {
                let size = usize::min(old_layout.size(), new_size);
                ptr::copy_nonoverlapping(ptr, new_ptr, size);
                GlobalAlloc::dealloc(self, ptr, old_layout);
            }

            new_ptr
        }
    }
}
