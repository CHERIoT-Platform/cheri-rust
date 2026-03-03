#[test]
// FIXME:
// error[E0080]: reading memory at alloc72[0x8..0xc], but memory is uninitialized at [0x8..0xc], and this operation requires initialized memory
//   --> coretests/tests/convert.rs:18:35
//    |
// 18 |     const BAR: Vec<String> = into(Vec::new());
//    |                                   ^^^^^^^^^^ evaluation of `convert::convert::BAR` failed inside this call
//    |
//    = note: the raw bytes of the constant (size: 16, align: 8) {
//                08 00 00 00 00 00 00 00 __ __ __ __ __ __ __ __ │ ........░░░░░░░░
//            }
#[cfg(not(target_abi = "cheriot"))]
fn convert() {
    const fn from(x: i32) -> i32 {
        i32::from(x)
    }

    const FOO: i32 = from(42);
    assert_eq!(FOO, 42);

    const fn into(x: Vec<String>) -> Vec<String> {
        x.into()
    }

    const BAR: Vec<String> = into(Vec::new());
    assert_eq!(BAR, Vec::<String>::new());
}

#[test]
fn into_as_try_into() {
    struct A;
    struct B;

    impl Into<B> for A {
        fn into(self) -> B {
            B
        }
    }

    // This wouldn't compile if the `TryInto`/`TryFrom` blanket impls used
    // `U: From<T>` instead of `T: Into<U>`
    let Ok(B) = A.try_into();
    let Ok(B) = B::try_from(A);
}
