-- Copyright Microsoft and CHERIoT Contributors.
-- SPDX-License-Identifier: MIT

set_project("Rust and compartments")

local rtosdir = path.absolute("./cheriot-rtos")
local sdkdir = rtosdir .. "/sdk"

includes(sdkdir)
set_toolchains("cheriot-clang")

option("board")
   set_default("sail")

compartment("my_app")
	  -- The following line is the important bit.
    add_files("./crates/my_app/Cargo.toml", { rules = { "cheriot.rust.crate", override = true }, force = true, sourcekind = "cheriot.rust.crate" })
	  add_files("./rtos_shim.cc")

compartment("log")
	  -- The following line is the important bit.
    add_files("./crates/log/Cargo.toml", { rules = { "cheriot.rust.crate", override = true }, force = true, sourcekind = "cheriot.rust.crate" })
	  add_files("./rtos_shim.cc")

firmware("rust-and-compartments")

    -- RTOS-provided libraries
    add_deps("freestanding", "debug", "stdio")

    -- Our compartments
    add_deps("log")
    add_deps("my_app")
    on_load(function(target)
        target:values_set("threads", {
            {
            compartment = "my_app",
            priority = 1,
            entry_point = "call_rust",
            stack_size = 0x1F00,
            trusted_stack_frames = 6
          },
        }, {expand = false})
    end)
