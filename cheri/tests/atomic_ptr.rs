#![no_std]

extern crate alloc;
extern crate cheriot;

#[no_mangle]
extern "C" fn test_atomic_ptr() -> i32 {
    use core::sync::atomic::AtomicPtr;
    let mut data = core::hint::black_box(5);
    let atomic_ptr = AtomicPtr::new(&mut data);
    assert_eq!(unsafe { *atomic_ptr.into_inner() }, 5);

    0
}
