#![no_std]

extern crate alloc;
extern crate cheriot;

#[no_mangle]
extern "C" fn test_smoke() -> i32 {
    cheriot::println!("Hello from 🍒 Rust!");
    0
}
