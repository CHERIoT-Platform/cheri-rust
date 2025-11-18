#![no_std]

extern crate alloc;
extern crate cheriot;

use core::cell::UnsafeCell;

#[no_mangle]
extern "C" fn test_unsafecell() -> i32 {
    core::hint::black_box(unsafe {
        let x = UnsafeCell::new(0);
        *(&x as *const UnsafeCell<i32> as *const i32)
    })
}
