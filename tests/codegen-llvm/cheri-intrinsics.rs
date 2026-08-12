//@ compile-flags: -Copt-level=0
//@ only-riscv32cheriot-unknown-cheriotrtos
// ignore-tidy-linelength

#![no_std]
#![feature(core_intrinsics)]
#![feature(stdarch_cheri)]
#![crate_type = "lib"]

#[no_mangle]
pub unsafe fn cheri_intrinsics(x: u32) {
    let mut nil = core::ptr::null::<()>();
    // For some optimisations to happen, address_get doesn't lower to the cheri.cap.addr.get intrinsics. Instead,
    // it lowers to a `ptrtoint`:
    // CHECK: ptrtoint
    _ = core::arch::cheri::address_get(nil);

    // Address increment is inlined, and it just results in a gep.
    // CHECK: getelementptr
    let new_nil =
        core::arch::cheri::address_increment(core::hint::black_box(nil), core::hint::black_box(0));
    core::ptr::write_volatile(&mut nil as *mut _, new_nil);
    _ = core::arch::cheri::address_set(nil, 0);
    _ = core::arch::cheri::base_get(nil);
    _ = core::arch::cheri::bounds_set(nil, 0);
    _ = core::arch::cheri::bounds_set_exact(nil, 0);
    _ = core::arch::cheri::is_equal_exact(nil, nil);
    _ = core::arch::cheri::length_get(nil);
    _ = core::arch::cheri::permissions_and(nil, 0);
    _ = core::arch::cheri::permissions_get(nil);
    _ = core::arch::cheri::representable_alignment_mask(0);
    _ = core::arch::cheri::round_representable_length(0);
    _ = core::arch::cheri::seal(nil, nil);
    _ = core::arch::cheri::subset_test(nil, nil);
    _ = core::arch::cheri::tag_clear(nil);
    _ = core::arch::cheri::tag_get(nil);
    _ = core::arch::cheri::top_get(nil);
    _ = core::arch::cheri::type_get(nil);
    _ = core::arch::cheri::unseal(nil, nil);
}

// Since some functions will be inlined and others won't, we simply check for the intrinsics to be declared.

// CHECK: declare i1 @llvm.cheri.cap.subset.test(ptr addrspace(200), ptr addrspace(200)) addrspace(200)

// CHECK: declare i1 @llvm.cheri.cap.equal.exact(ptr addrspace(200), ptr addrspace(200)) addrspace(200)

// CHECK: declare ptr addrspace(200) @llvm.cheri.cap.seal(ptr addrspace(200), ptr addrspace(200)) addrspace(200)

// CHECK: declare ptr addrspace(200) @llvm.cheri.cap.unseal(ptr addrspace(200), ptr addrspace(200)) addrspace(200)

// CHECK: declare i1 @llvm.cheri.cap.tag.get(ptr addrspace(200)) addrspace(200)

// CHECK: declare ptr addrspace(200) @llvm.cheri.cap.tag.clear(ptr addrspace(200)) addrspace(200)

// CHECK: declare ptr addrspace(200) @llvm.cheri.cap.bounds.set.i32(ptr addrspace(200), i32) addrspace(200)

// CHECK: declare i32 @llvm.cheri.cap.length.get.i32(ptr addrspace(200)) addrspace(200)

// CHECK: declare ptr addrspace(200) @llvm.cheri.cap.address.set.i32(ptr addrspace(200), i32) addrspace(200)

// CHECK: declare ptr addrspace(200) @llvm.cheri.cap.perms.and.i32(ptr addrspace(200), i32) addrspace(200)

// CHECK: declare i32 @llvm.cheri.cap.perms.get.i32(ptr addrspace(200)) addrspace(200)

// CHECK: declare ptr addrspace(200) @llvm.cheri.cap.bounds.set.exact.i32(ptr addrspace(200), i32) addrspace(200)

// CHECK: declare i32 @llvm.cheri.cap.top.get.i32(ptr addrspace(200)) addrspace(200)

// CHECK: declare i32 @llvm.cheri.cap.base.get.i32(ptr addrspace(200)) addrspace(200)

// CHECK: declare i32 @llvm.cheri.cap.type.get.i32(ptr addrspace(200)) addrspace(200)

// CHECK: declare i32 @llvm.cheri.round.representable.length.i32(i32) addrspace(200)

// CHECK: declare i32 @llvm.cheri.representable.alignment.mask.i32(i32) addrspace(200)
