use crate::spec::{
    Arch, Cc, CfgAbi, LinkerFlavor, Lld, LlvmAbi, Os, PanicStrategy, RelocModel, Target,
    TargetMetadata, TargetOptions, cvs,
};

pub(crate) fn target() -> Target {
    Target {
        // The below `data_layout` is explicitly specified by the ilp32e ABI in LLVM. See also
        // `options.llvm_abiname`.
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128-pf200:64:64:64:32-A200-P200-G200".into(),
        llvm_target: "riscv32-unknown-cheriotrtos".into(),
        metadata: TargetMetadata {
            description: Some("CHERIoT RISC-V (RV32E ISA)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 64,
        arch: Arch::RiscV32,

        options: TargetOptions {
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: None,
            cpu: "cheriot".into(),
            cfg_abi: CfgAbi::CHERIoT,
            llvm_abiname: LlvmAbi::CHERIoT,
            max_atomic_width: Some(64),
            atomic_cas: true,
            features: "+32bit,+c,+e,+m,+xcheriot".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            families: cvs!["cheri", "cheriot"],
            os: Os::CHERIoTRTOS,
            is_like_cheri: true,
            executables: false,
            default_address_space: rustc_abi::AddressSpace(200),
            ..Default::default()
        },
    }
}
