RUSTC_SYSROOT="./build/host/stage1" \
RUSTC="./build/host/stage1/bin/rustc" \
RUSTBUILD_NATIVE_DIR="./build/riscv32cheriot-unknown-cheriotrtos/native" \
CARGO_TARGET_RISCV32CHERIOT_UNKNOWN_CHERIOTRTOS_LINKER="$LINKER" \
CARGO_TARGET_RISCV32CHERIOT_UNKNOWN_CHERIOTRTOS_RUNNER="$RUNNER" \
RUSTC_BOOTSTRAP="1" \
./build/host/stage0/bin/cargo test \
    --manifest-path "./library/alloc/Cargo.toml" \
    --target "riscv32cheriot-unknown-cheriotrtos" \
    --release \
    -p "coretests" \
    --tests \
    --features "compiler-builtins-mem coretests/partial_test coretests/test_$1" \
    -Zno-embed-metadata
