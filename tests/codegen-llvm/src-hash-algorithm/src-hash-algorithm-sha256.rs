//@ compile-flags: -g -Z src-hash-algorithm=sha256 -Copt-level=0

#![crate_type = "lib"]
#![no_std]

pub fn test() {}
// CHECK: checksumkind: CSK_SHA256
