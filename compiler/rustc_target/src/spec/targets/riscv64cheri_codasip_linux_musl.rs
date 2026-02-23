use crate::spec::{
    Arch, Cc, Env, LinkSelfContainedDefault, LinkerFlavor, Lld, LlvmAbi, Os, PanicStrategy,
    RelocModel, Target, TargetMetadata, TargetOptions, crt_objects, cvs,
};

pub(crate) fn target() -> Target {
    Target {
        data_layout: "e-m:e-p
           f200:128:128:128:64-p:64:64-i64:64-i128:128-n32:64-S128-A200-P200-G200"
            .into(),
        llvm_target: "riscv64-codasip-linux-musl".into(),
        metadata: TargetMetadata {
            description: Some("Codasip CHERI RISC-V Linux".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 128,
        arch: Arch::RiscV64,

        options: TargetOptions {
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: None,
            cpu: "codasip-x730-spark".into(),
            llvm_abiname: LlvmAbi::L64pc128d,
            max_atomic_width: Some(128),
            features: "+64bit,+m,+a,+f,+d,+c,+zicsr,+zifencei,+zcheripurecap,+cap-mode".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            families: cvs!["unix", "cheri"],
            os: Os::Linux,
            vendor: "codasip".into(),
            env: Env::Musl,
            is_like_cheri: true,
            executables: true,
            default_address_space: rustc_abi::AddressSpace(200),
            pre_link_objects_self_contained: crt_objects::pre_musl_self_contained(),
            post_link_objects_self_contained: crt_objects::post_musl_self_contained(),
            link_self_contained: LinkSelfContainedDefault::InferredForMusl,
            address_width: Some(64),
            ..Default::default()
        },
    }
}
