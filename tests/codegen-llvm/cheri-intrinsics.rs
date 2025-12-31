//@ compile-flags: -Copt-level=0
//@ only-riscv32cheriot-unknown-cheriotrtos

#![no_std]
#![feature(core_intrinsics)]
#![crate_type = "lib"]

// CHECK-LABEL: @cheri_intrinsics(
#[no_mangle]
pub unsafe fn cheri_intrinsics(x: u32) {
    let nil = core::ptr::null();

    // CHECK: llvm.cheri.cap.address.get.
    _ = core::intrinsics::cheri::cheri_address_get(nil);

    // _ = core::intrinsics::cheri::cheri_address_increment(nil, 0);

    // CHECK: llvm.cheri.cap.address.set.
    _ = core::intrinsics::cheri::cheri_address_set(nil, 0);

    // CHECK: llvm.cheri.cap.base.get.
    _ = core::intrinsics::cheri::cheri_base_get(nil);

    // CHECK: llvm.cheri.cap.bounds.set.
    _ = core::intrinsics::cheri::cheri_bounds_set(nil, 0);

    // CHECK: llvm.cheri.cap.bounds.set.exact.
    _ = core::intrinsics::cheri::cheri_bounds_set_exact(nil, 0);

    // CHECK: llvm.cheri.cap.equal.exact
    _ = core::intrinsics::cheri::cheri_is_equal_exact(nil, nil);

    // CHECK: llvm.cheri.cap.length.get.
    _ = core::intrinsics::cheri::cheri_length_get(nil);

    // CHECK: llvm.cheri.cap.perms.and
    _ = core::intrinsics::cheri::cheri_permissions_and(nil, 0);

    // CHECK: llvm.cheri.cap.perms.get
    _ = core::intrinsics::cheri::cheri_permissions_get(nil);

    // CHECK: llvm.cheri.representable.alignment.mask
    _ = core::intrinsics::cheri::cheri_representable_alignment_mask(0);

    // CHECK: llvm.cheri.round.representable.length
    _ = core::intrinsics::cheri::cheri_round_representable_length(0);

    // CHECK: llvm.cheri.cap.seal
    _ = core::intrinsics::cheri::cheri_seal(nil, nil);

    // CHECK: llvm.cheri.cap.subset.test
    _ = core::intrinsics::cheri::cheri_subset_test(nil, nil);

    // CHECK: llvm.cheri.cap.tag.clear
    _ = core::intrinsics::cheri::cheri_tag_clear(nil);

    // CHECK: llvm.cheri.cap.tag.get
    _ = core::intrinsics::cheri::cheri_tag_get(nil);

    // CHECK: llvm.cheri.cap.top.get
    _ = core::intrinsics::cheri::cheri_top_get(nil);

    // CHECK: llvm.cheri.cap.type.get
    _ = core::intrinsics::cheri::cheri_type_get(nil);

    // CHECK: llvm.cheri.cap.unseal
    _ = core::intrinsics::cheri::cheri_unseal(nil, nil);
}
