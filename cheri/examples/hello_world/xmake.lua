sdkdir = path.absolute("./cheriot-rtos/sdk")

includes(sdkdir)

set_toolchains("cheriot-clang")

option("board")
  set_default("sail")

compartment("test_runner")
    add_deps("freestanding", "string", "crt", "cxxrt", "atomic_fixed", "compartment_helpers", "debug", "softfloat")
    add_deps("message_queue", "locks", "event_group", "cheriot.allocator")
    add_deps("stdio")
    add_deps("strtol")
	add_files("runner.cc")
	add_ldflags("-L./target/riscv32cheriot-unknown-cheriotrtos/release", {force = true})
    add_ldflags("-lhello_world", {force = true})

firmware("test")
    add_deps("test_runner")
    on_load(function(target)
        target:values_set("board", "$(board)")
        target:values_set("threads", {
            {
            compartment = "test_runner",
            priority = 1,
            entry_point = "run",
            stack_size = 0x1F00,
            trusted_stack_frames = 6
          },
        }, {expand = false})
    end)
