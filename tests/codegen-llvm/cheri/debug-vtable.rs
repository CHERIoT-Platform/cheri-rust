//@ revisions: cheriot
//@ [cheriot] compile-flags: --target riscv32cheriot-unknown-cheriotrtos -Cdebuginfo=2 -Copt-level=0 -Csymbol-mangling-version=v0
//@ [cheriot] needs-llvm-components: riscv

#![crate_type = "lib"]
#![no_std]

extern crate alloc;
use alloc::boxed::Box;

// cheriot: @ptable
// Force emission for debuginfo for usize and *const() early..
pub static mut XYZ: Option<(usize, *const ())> = None;

pub struct Foo {
    dropme: alloc::string::String,
}

pub trait SomeTrait {
    fn method1(&self) -> u32;
    fn method2(&self) -> u32;
}

impl SomeTrait for Foo {
    fn method1(&self) -> u32 {
        1
    }
    fn method2(&self) -> u32 {
        2
    }
}

pub trait SomeTraitWithGenerics<T, U> {
    fn method1(&self) -> (T, U);
}

impl SomeTraitWithGenerics<u64, i8> for Foo {
    fn method1(&self) -> (u64, i8) {
        (1, 2)
    }
}

pub fn foo(x: &Foo) -> (u32, (u64, i8), &dyn Send) {
    let y: &dyn SomeTrait = x;
    let z: &dyn SomeTraitWithGenerics<u64, i8> = x;
    (y.method1(), z.method1(), x as &dyn Send)
}

// Constructing the debuginfo name for the FnOnce vtable below initially caused an ICE on MSVC
// because the trait type contains a late bound region that needed to be erased before the type
// layout for the niche enum `Option<&dyn Fn()>` could be computed.
pub fn bar() -> Box<dyn FnOnce(Option<&dyn Fn()>)> {
    Box::new(|_x: Option<&dyn Fn()>| {})
}

fn generic_closure<T: 'static>(x: T) -> Box<dyn FnOnce() -> T> {
    Box::new(move || x)
}

pub fn instantiate_generic_closures() -> (Box<dyn FnOnce() -> u32>, Box<dyn FnOnce() -> bool>) {
    (generic_closure(1u32), generic_closure(false))
}
