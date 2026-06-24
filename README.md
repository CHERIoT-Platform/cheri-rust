# The Evil Rust Programming Language - experimental CHERI\[oT\] port.

> [!WARNING]  
> This project is still in its early days, and must be considered experimental and not ready for production.
>
> This fork is not built or maintained by the Rust project proper, please don't complain to them if you have issues with it. 
> If you run into a bug using this compiler, please raise an issue in this repository or reach out in the [public CHERIoT group on Signal](https://signal.group/#CjQKIElxAs3t3MUEMOEmQEuMHRK4rErUk2xVeFzjAjFXAShzEhCK9qQwEMFKGLGZnCjrQ7zm). 

This is a fork of [Rust](https://www.rust-lang.org) which adds experimental
support for [CHERIoT](https://cheriot.org/), a [CHERI](https://en.wikipedia.org/wiki/Capability_Hardware_Enhanced_RISC_Instructions)-enabled RISC-V platform. As of now, CHERIoT is the only
platform we officially plan to support, but we strive to make our changes to
the compiler compatible with other CHERI platforms that may be added in the
future. 

If you are working on porting Rust to another CHERI platform, please
contact us, either on [Signal](https://signal.group/#CjQKIElxAs3t3MUEMOEmQEuMHRK4rErUk2xVeFzjAjFXAShzEhCK9qQwEMFKGLGZnCjrQ7zm)
or by opening a new discussion in this repository.

### The state of this project

As of late November 2025, we have a `rustc` that can produce
programs for the CHERIoT platform. This `rustc` can then compile the
`compiler-builtins`, `core` and `alloc` libraries. We are currently in the
process of testing the functionalities exposed by these libraries.

As of now, CHERIoT-specific bits such as accessing MMIO devices, shared
objects, defining compartments and using specific calling conventions are
planned but not implemented yet. To access these functionalities, your best
chance is to use FFI to call C(++) shims that do what the Rust compiler
can't currently do. 

### Building the compiler

We try to keep building the compiler an easy and approachable task.
In one line:
```
git clone https://github.com/CHERIoT-Platform/cheri-rust.git &&\
    cd cheri-rust &&\
    ./cheri/gen_bootstrap.py --build-clang &&\
    ./x build compiler std --target=riscv32cheriot-unknown-cheriotrtos
```

The `./x` command will download a `rustc` to bootstrap Rust, compile LLVM and proceed to compile the compiler and the supported bits of the standard library.

If this process fails, please raise an issue so that we can try to fix the problems you encountered.

You can then use `rustup` to create a new toolchain with the name you prefer:
```
rustup toolchain link 'cheri' build/host/stage1
```
Notice that this process does not generate `cargo` and `rust-analyzer` as well. You'll have to build them with `./x`...
```
 ./x build tools/cargo tools/rust-analyzer
```
...and then link the results manually to your `$RUSTUP_HOME`:
```
ln -s $PWD/build/host/stage1-tools-bin/cargo $RUSTUP_HOME/toolchains/cheri/bin &&\
ln -s $PWD/build/host/stage1-tools-bin/rust-analyzer $RUSTUP_HOME/toolchains/cheri/bin
```

### Now what?

Keep in mind that this is an experimental project, and anything can break at any time.

As of now, the process of getting a final image for CHERIoT will entail
1. Providing an umangled extern "C" API in your Rust project;
2. Compiling your project to a static library;
3. Having a C(++) shim that calls the API exposed from Rust;
4. Linking everything together with the [CHERIoT RTOS](http://github.com/CHERIoT-Platform/cheriot-rtos);
5. Flashing and/or running the resulting image with a [CHERIoT simulator](https://github.com/CHERIoT-Platform/cheriot-sail) or a [Sonata](https://lowrisc.github.io/sonata-software/doc/getting-started.html) board.

For an example of all the steps above, take a look at the [cheri/examples/hello_world](./cheri/examples/hello_world) directory. 

<p align="center">
</br></br>
Something went wrong? Please <a href="https://github.com/CHERIoT-Platform/cheri-rust/issues/new?template=bug_report.md">raise an issue</a>
or contact us on the <a href="https://signal.group/#CjQKIElxAs3t3MUEMOEmQEuMHRK4rErUk2xVeFzjAjFXAShzEhCK9qQwEMFKGLGZnCjrQ7zm">public CHERIoT group on Signal</a>
</p>
