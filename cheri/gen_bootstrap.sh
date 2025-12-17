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

# Check if we're running CI for a PR to main.
if [ -n "${CIRRUS_BASE_BRANCH+set}" ]; then
    if [ "$CIRRUS_BASE_BRANCH" == "main" ]; then
        CHANNEL="nightly"
        # We are seeing some faults unrelated to our changes when CI runs from
        # main, specifically tied to gcc. When running like that (CI from main)
        # we want to restrict the backend to LLVM and cranelift only, so that
        # we don't get stuck on things that aren't our fault. Ideally, we
        # should restore the normal behaviour once the error is fixed.
        CODEGEN_BACKENDS="codegen-backends = [\"llvm\",\"cranelift\"]"
    else
        # TODO: change this to `dev` once we add rustfmt to the beta branch, so
        # that fmt --check is part of the tests.
        CHANNEL="beta"
    fi
# Otherwise we may be running for a direct push to main.
elif [ -n "${CIRRUS_BRANCH+set}" ]; then
    if [ "$CIRRUS_BRANCH" == "main" ]; then
        CHANNEL="nightly"

        # See comment above.
        CODEGEN_BACKENDS="codegen-backends = [\"llvm\",\"cranelift\"]"
    else
        CHANNEL="beta"
    fi
else
        CHANNEL="beta"
fi

if [ -e "$FILE" ]; then
  echo "$FILE already exists!"
  exit 1
else
cat > "$FILE" <<- EOF

# See bootstrap.example.toml for documentation of available options
#
profile = "compiler"  # Includes one of the default files in src/bootstrap/defaults
change-id = "ignore"

[build]
ccache = true

[rust]
channel = "$CHANNEL"
$CODEGEN_BACKENDS
#debug = true
#debuginfo-level = 2
std-features = ["compiler-builtins-mem"]

[llvm]
targets = "all"
experimental-targets = ""
download-ci-llvm = false
build-config = {$CUSTOM_CMAKE_FLAGS}
EOF
fi
