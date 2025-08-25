#!/bin/bash

FILE="bootstrap.toml"

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
#channel = "nightly"
#codegen-backends = ["llvm"]
#debug = true
#debuginfo-level = 2
std-features = ["compiler-builtins-mem"]

[llvm]
targets = "all"
experimental-targets = ""
download-ci-llvm = false
build-config = {CMAKE_C_COMPILER="clang", CMAKE_CXX_COMPILER="clang++"}

EOF
fi
