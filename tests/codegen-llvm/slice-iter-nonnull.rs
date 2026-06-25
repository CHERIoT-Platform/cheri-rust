//@ compile-flags: -Copt-level=3
//@ needs-deterministic-layouts
#![crate_type = "lib"]
#![feature(exact_size_is_empty)]
#![no_std]

// The slice iterator used to `assume` that the `start` pointer was non-null.
// That ought to be unneeded, though, since the type is `NonNull`, so this test
// confirms that the appropriate metadata is included to denote that.

// It also used to `assume` the `end` pointer was non-null, but that's no longer
// needed as the code changed to read it as a `NonNull`, and thus gets the
// appropriate `!nonnull` annotations naturally.

// CHECK-LABEL: @slice_iter_next(
#[no_mangle]
pub fn slice_iter_next<'a>(it: &mut core::slice::Iter<'a, u32>) -> Option<&'a u32> {
    // CHECK: %[[START:.+]] = load ptr[[ADDRSPACE]], ptr[[ADDRSPACE]] %it,
    // CHECK-SAME: !nonnull
    // CHECK-SAME: !noundef
    // CHECK: %[[ENDP:.+]] = getelementptr inbounds{{( nuw)?}} i8, ptr[[ADDRSPACE]] %it, {{i32 4|i32 8|i64 8}}
    // CHECK: %[[END:.+]] = load ptr[[ADDRSPACE]], ptr[[ADDRSPACE]] %[[ENDP]]
    // CHECK-SAME: !nonnull
    // CHECK-SAME: !noundef
    // CHECK: icmp eq ptr[[ADDRSPACE]] %[[START]], %[[END]]

    // CHECK: store ptr[[ADDRSPACE]]{{.+}}, ptr[[ADDRSPACE]] %it,

    it.next()
}

// CHECK-LABEL: @slice_iter_next_back(
#[no_mangle]
pub fn slice_iter_next_back<'a>(it: &mut core::slice::Iter<'a, u32>) -> Option<&'a u32> {
    // CHECK: %[[ENDP:.+]] = getelementptr inbounds{{( nuw)?}} i8, ptr[[ADDRSPACE]] %it, {{i32 4|i32 8|i64 8}}
    // CHECK: %[[END:.+]] = load ptr[[ADDRSPACE]], ptr[[ADDRSPACE]] %[[ENDP]]
    // CHECK-SAME: !nonnull
    // CHECK-SAME: !noundef
    // CHECK: %[[START:.+]] = load ptr[[ADDRSPACE]], ptr[[ADDRSPACE]] %it,
    // CHECK-SAME: !nonnull
    // CHECK-SAME: !noundef
    // CHECK: icmp eq ptr[[ADDRSPACE]] %[[START]], %[[END]]

    // CHECK: store ptr[[ADDRSPACE]]{{.+}}, ptr[[ADDRSPACE]] %[[ENDP]],

    it.next_back()
}

// The slice iterator `new` methods used to `assume` that the pointer is non-null,
// but passing slices already requires that, to the extent that LLVM actually
// removed the `call @llvm.assume` anyway.  These tests just demonstrate that the
// attribute is there, and confirms adding the assume back doesn't do anything.

// CHECK-LABEL: @slice_iter_new
// CHECK-SAME: (ptr[[ADDRSPACE]] noalias nofree noundef nonnull {{.+}} %slice.0, {{.+}} noundef range({{.+}}) %slice.1)
#[no_mangle]
pub fn slice_iter_new(slice: &[u32]) -> core::slice::Iter<'_, u32> {
    // CHECK-NOT: slice
    // CHECK: %[[END:.+]] = getelementptr inbounds{{( nuw)?}} {{i32|\[4 x i8\]}}{{.+}} %slice.0{{.+}} %slice.1
    // CHECK-NOT: slice
    // CHECK: insertvalue {{.+}} ptr[[ADDRSPACE]] %slice.0, 0
    // CHECK-NOT: slice
    // CHECK: insertvalue {{.+}} ptr[[ADDRSPACE]] %[[END]], 1
    // CHECK-NOT: slice
    // CHECK: }
    slice.iter()
}

// CHECK-LABEL: @slice_iter_mut_new
// CHECK-SAME: (ptr[[ADDRSPACE]] noalias nofree noundef nonnull {{.+}} %slice.0, {{.+}} noundef range({{.+}}) %slice.1)
#[no_mangle]
pub fn slice_iter_mut_new(slice: &mut [u32]) -> core::slice::IterMut<'_, u32> {
    // CHECK-NOT: slice
    // CHECK: %[[END:.+]] = getelementptr inbounds{{( nuw)?}} {{i32|\[4 x i8\]}}{{.+}} %slice.0{{.+}} %slice.1
    // CHECK-NOT: slice
    // CHECK: insertvalue {{.+}} ptr[[ADDRSPACE]] %slice.0, 0
    // CHECK-NOT: slice
    // CHECK: insertvalue {{.+}} ptr[[ADDRSPACE]] %[[END]], 1
    // CHECK-NOT: slice
    // CHECK: }
    slice.iter_mut()
}

// CHECK-LABEL: @slice_iter_is_empty
#[no_mangle]
pub fn slice_iter_is_empty(it: &core::slice::Iter<'_, u32>) -> bool {
    // CHECK: %[[ENDP:.+]] = getelementptr inbounds{{( nuw)?}} i8, ptr[[ADDRSPACE]] %it, {{i32 4|i32 8|i64 8}}
    // CHECK: %[[END:.+]] = load ptr[[ADDRSPACE]], ptr[[ADDRSPACE]] %[[ENDP]]
    // CHECK-SAME: !nonnull
    // CHECK-SAME: !noundef
    // CHECK: %[[START:.+]] = load ptr[[ADDRSPACE]], ptr[[ADDRSPACE]] %it,
    // CHECK-SAME: !nonnull
    // CHECK-SAME: !noundef

    // CHECK: %[[RET:.+]] = icmp eq ptr[[ADDRSPACE]] %[[START]], %[[END]]
    // CHECK: ret i1 %[[RET]]
    it.is_empty()
}

// CHECK-LABEL: @slice_iter_len
#[no_mangle]
pub fn slice_iter_len(it: &core::slice::Iter<'_, u32>) -> usize {
    // CHECK: %[[ENDP:.+]] = getelementptr inbounds{{( nuw)?}} i8, ptr[[ADDRSPACE]] %it, {{i32 4|i32 8|i64 8}}
    // CHECK: %[[END:.+]] = load ptr[[ADDRSPACE]], ptr[[ADDRSPACE]] %[[ENDP]]
    // CHECK-SAME: !nonnull
    // CHECK-SAME: !noundef
    // CHECK: %[[START:.+]] = load ptr[[ADDRSPACE]], ptr[[ADDRSPACE]] %it,
    // CHECK-SAME: !nonnull
    // CHECK-SAME: !noundef

    // CHECK: ptrtoint
    // CHECK: ptrtoint
    // CHECK: sub nuw
    // CHECK: lshr exact
    it.len()
}
