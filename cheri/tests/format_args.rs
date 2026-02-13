#![no_std]

extern crate alloc;
extern crate cheriot;

#[no_mangle]
extern "C" fn test_format_args() -> i32 {
    assert_eq!(format_args!("hello").as_str(), Some("hello"));
    assert_eq!(format_args!("").as_str(), Some(""));
    assert_eq!(format_args!("{:?}", cheriot::rand::rand()).as_str(), None);
    let x = 42;
    assert_eq!(alloc::format!("{x:#b}"), "0b101010");
    0
}
