//@ compile-flags: -g -Z src-hash-algorithm=md5 -Copt-level=0

#![crate_type = "lib"]
#![no_std]

pub fn test() {}
// CHECK: checksumkind: CSK_MD5
