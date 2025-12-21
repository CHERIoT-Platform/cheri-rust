//@ add-minicore
//@ compile-flags: -Copt-level=3
#![feature(c_variadic)]
#![crate_type = "lib"]
#![no_std]

// Check that `%args` explicitly has its lifetime start and end. Being explicit can improve
// instruction and register selection, see e.g. https://github.com/rust-lang/rust/pull/144549

#[unsafe(no_mangle)]
unsafe extern "C" fn variadic(a: f64, mut args: ...) -> f64 {
    // CHECK: call void @llvm.lifetime.start.[[PSPACE:p[0-9]+]]({{(i64 [0-9]+, )?}}ptr[[ADDRSPACE]] nonnull %args)
    // CHECK: call void @llvm.va_start.[[PSPACE]](ptr[[ADDRSPACE]] nonnull %args)

    let b = args.arg::<f64>();
    let c = args.arg::<f64>();

    a + b + c

    // CHECK: call void @llvm.va_end.[[PSPACE]](ptr[[ADDRSPACE]] nonnull %args)
    // CHECK: call void @llvm.lifetime.end.[[PSPACE]]({{(i64 [0-9]+, )?}}ptr[[ADDRSPACE]] nonnull %args)
}
