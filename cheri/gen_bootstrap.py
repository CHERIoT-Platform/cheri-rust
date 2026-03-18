#! /usr/bin/env python3

# Generate a bootstrap.toml file.
# This script will be used by users and in CI, and we need to take into account
# multiple requirements:
#   * In CI:
#       + We always want to compile LLVM.
#       + LLVM must have assertions on.
#       + CI could run from a direct push to a branch or from a PR.
#           - PR: if the base branch is 'main', 'channel' must be 'nightly'.
#           - PR: if the base branch is 'beta', 'channel' must be 'dev'.
#       + We need to add the `CMAKE_C_COMPILER` and `CMAKE_CXX_COMPILER` flags.
#   * For users:
#       + Users might want to use a local installation of LLVM without having to recompile it.
#           - To specify this, users will have to use the `--llvm-config-bin=<path>` flag.
#       + The 'channel' is always 'dev'.
#   * Both:
#       + If we compile LLVM, we might also want to compile `clang`. This is set with the `--build-clang` flag, which is incompatible with `--llvm-config-bin`.
from typing import Any

import argparse
import subprocess
import os


# No support to _write_ TOML files in Python's stdlib, so we gotta roll our own small writer.
def dict_to_toml_lines(obj: dict) -> list[str]:

    def value_to_toml(v) -> str:
        assert isinstance(v, str) or isinstance(v, bool)
        if isinstance(v, str):
            return f'"{v}"'
        if isinstance(v, bool):
            if v:
                return "true"
            else:
                return "false"

    lines = []
    for k, v in obj.items():
        if isinstance(v, str) or isinstance(v, bool):
            toml_v = value_to_toml(v)
            lines.append(f"{k} = {toml_v}")

        elif isinstance(v, list):
            lines.append(f"{k} = [{', '.join([value_to_toml(i) for i in v])}]")

        elif isinstance(v, dict):
            lines.append(f"\n[{k}]")
            lines.extend(dict_to_toml_lines(v))

        else:
            assert False, ("Unsupported TOML key-value pair: ", k, v, type(v))

    return lines


def to_toml(obj: dict) -> str:
    return "\n".join(dict_to_toml_lines(obj))


# Small helper to avoid always doing "if self.verbose:".
class Logger:
    verbose: bool

    def __init__(self, verbose: bool):
        self.verbose = verbose

    def print(self, *args):
        if self.verbose:
            print(*args)


# Keeps track of the environment we're running in.
class Generator:
    in_ci: bool
    ci_branch: str | None = None

    build_clang: bool

    use_llvm_config: bool
    llvm_config_bin: str | None = None

    ccache: str | bool

    enable_optimize_for_size: bool

    def __repr__(self):
        return str(self.__dict__)

    def __init__(self, args):
        self.build_clang = args.build_clang
        self.use_llvm_config = isinstance(args.llvm_config_bin, str)
        self.llvm_config_bin = args.llvm_config_bin
        self.enable_optimize_for_size = args.optimize_for_size
        self.in_ci = os.getenv("CIRRUS_TASK_ID") is not None
        if self.in_ci:
            self.ci_branch = os.getenv("CIRRUS_BASE_BRANCH") or os.getenv(
                "CIRRUS_BRANCH"
            )

        try:
            self.ccache = subprocess.check_output(["which", "ccache"]).decode().strip()
        except FileNotFoundError:
            self.ccache = False

    def build(self) -> str:
        channel = "dev"

        if self.in_ci:
            if self.ci_branch == "main":
                channel = "nightly"

        std_features = ["compiler-builtins-mem"]

        if self.enable_optimize_for_size:
            std_features.append("optimize_for_size")

        ret: dict[str, Any] = {
            "profile": "compiler",
            "change-id": "ignore",
            "build": {
                "ccache": self.ccache,
                "rustfmt": "./build/host/stage1/bin/rustfmt",
            },
            "rust": {
                "channel": channel,
                "std-features": std_features,
            },
            "llvm": {"download-ci-llvm": False},
        }

        if self.use_llvm_config:
            assert isinstance(self.llvm_config_bin, str)
            host = (
                subprocess.check_output([self.llvm_config_bin, "--host-target"])
                .decode()
                .strip()
            )

            ret[f"target.{host}"] = {"llvm-config": self.llvm_config_bin}
            ret["target.riscv32cheriot-unknown-cheriotrtos"] = {
                "llvm-config": self.llvm_config_bin
            }
        else:
            ret["llvm"]["targets"] = "all"
            ret["llvm"]["experimental-targets"] = ""

            extra_cmake_flags = {}

            if self.build_clang:
                extra_cmake_flags["LLVM_ENABLE_PROJECTS"] = "clang;lld"

            if self.in_ci:
                ret["llvm"]["assertions"] = True
                extra_cmake_flags["CMAKE_C_COMPILER"] = "clang"
                extra_cmake_flags["CMAKE_CXX_COMPILER"] = "clang++"

            ret["llvm"]["build-config"] = extra_cmake_flags

        return to_toml(ret)


# Generate the parser for the CLI arguments.
def cli():
    parser = argparse.ArgumentParser(
        prog="gen_bootstrap.py",
        description="Generates a `bootstrap.toml` to be used in CI or by users. ",
    )
    parser.add_argument(
        "-o",
        "--output",
        help="The name of the file to write the generated bootstrap configuration to.",
        default="bootstrap.toml",
    )
    parser.add_argument(
        "-v",
        "--verbose",
        action="store_true",
        help="Print useful messages while generating the file.",
    )
    parser.add_argument(
        "--optimize-for-size",
        action="store_true",
        help="Enable the `optimize_for_size` std feature.",
    )
    parser.add_argument(
        "-f",
        "--force",
        action="store_true",
        help="Overwrite the output file if it already exists.",
    )
    should_build_llvm = parser.add_mutually_exclusive_group()
    should_build_llvm.add_argument(
        "--build-clang",
        action="store_true",
        help="Whether to build `clang` when building LLVM.",
    )
    should_build_llvm.add_argument(
        "--llvm-config-bin",
        help="The path to the `llvm-config` binary to use instead of building LLVM.",
    )
    return parser


def main():

    parser = cli()
    args = parser.parse_args()

    log = Logger(args.verbose)
    log.print("got args:", args)

    gen = Generator(args)
    log.print("generator config:", gen)

    config = gen.build()

    log.print("generated config:\n", config)

    if args.output == "-":
        log.print("writing output to stdout")
        print(config)
    else:
        log.print("writing output to:", args.output)
        if os.path.exists(args.output) and not args.force:
            print(
                f"File {args.output} already exists! Use the `--force` flag to overwrite it."
            )
        else:
            with open(args.output, "wt+") as f:
                f.write(config)


if __name__ == "__main__":
    main()
