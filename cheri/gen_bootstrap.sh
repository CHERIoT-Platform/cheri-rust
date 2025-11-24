#!/bin/bash

FILE="bootstrap.toml"

if  [[ $1 = "--build-clang" ]]; then
    CUSTOM_CMAKE_FLAGS="LLVM_ENABLE_PROJECTS=\"clang;lld\""
fi

if  [ -n "${CIRRUS_TASK_ID+set}" ]; then
    CI_CMAKE_FLAGS="CMAKE_C_COMPILER=\"clang\", CMAKE_CXX_COMPILER=\"clang++\""
    if [ -n "${CUSTOM_CMAKE_FLAGS+set}" ]; then
        CUSTOM_CMAKE_FLAGS="$CI_CMAKE_FLAGS, $CUSTOM_CMAKE_FLAGS"
    else
        CUSTOM_CMAKE_FLAGS="$CI_CMAKE_FLAGS"
    fi
elif [ -z "${CUSTOM_CMAKE_FLAGS+set}" ]; then
        CUSTOM_CMAKE_FLAGS=""
fi

if [ -e "$FILE" ]; then
  echo "$FILE already exists!"
  exit 1
else
cat > "$FILE" <<- EOF

# See bootstrap.example.toml for documentation of available options
#
profile = "compiler"  # Includes one of the default files in src/bootstrap/defaults
change-id = 140732

[build]
ccache = true

[rust]
channel = "beta"
#codegen-backends = ["llvm"]
#debug = true
#debuginfo-level = 2
std-features = ["compiler-builtins-mem"]

[llvm]
targets = "all"
experimental-targets = ""
download-ci-llvm = false
build-config = {$CUSTOM_CMAKE_FLAGS}

[target.riscv32cheriot-unknown-cheriotrtos]
no-std = true

EOF
fi
