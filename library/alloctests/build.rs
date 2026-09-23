use std::env;

fn main() {
    if env::var("CARGO_FEATURE_NEEDS_MATH").is_ok() {
        println!("cargo::rustc-link-arg-tests=--needs-math")
    }

    if env::var("CARGO_FEATURE_NEEDS_SOFTFLOAT").is_ok() {
        println!("cargo::rustc-link-arg-tests=--needs-softfloat");
    }
}
