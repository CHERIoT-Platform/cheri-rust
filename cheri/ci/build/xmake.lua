includes("cheriot-rtos/sdk")

set_toolchains("cheriot-clang")

option("board")
    set_default("sail")

compartment("test_runner")
    add_deps("atomic","debug","freestanding","softfloat","stdio")
    add_files("runner.cc")
    before_link(function(target)
        local objectfiles = target:objectfiles()
        for line in io.lines("rust_objects.txt") do
            table.insert(objectfiles, line)
        end
    end)

firmware("test_fw")
    add_deps("test_runner")
    on_load(function(target)
        target:values_set("board", "$(board)")
        target:values_set("threads", {
            {
                compartment = "test_runner",
                priority = 1,
                entry_point = "run",
                stack_size = 0x1F00,
                trusted_stack_frames = 12
            }
        }, { expand = false })
    end)
