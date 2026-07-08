use std::borrow::Cow;
use std::fmt;

use rustc_data_structures::fx::FxIndexSet;
use rustc_span::{Symbol, sym};

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};
use crate::spec::{RelocModel, Target};

def_reg_class! {
    RiscV RiscVInlineAsmRegClass {
        reg,
        creg,
        freg,
        vreg,
    }
}

impl RiscVInlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: super::InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<ModifierInfo> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<ModifierInfo> {
        None
    }

    pub fn supported_types(
        self,
        arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        match self {
            Self::reg => {
                if arch == InlineAsmArch::RiscV64 {
                    types! { _: I8, I16, I32, I64, F16, F32, F64; }
                } else {
                    types! { _: I8, I16, I32, F16, F32; }
                }
            }
            Self::creg => {
                if arch == InlineAsmArch::RiscV64 {
                    types! { _: I8, I16, I32, I64, I128, F16, F32, F64; }
                } else {
                    types! { _: I8, I16, I32, I64, F16, F32, F64; }
                }
            }
            // FIXME(f128): Add `q: F128;` once LLVM support the `Q` extension.
            Self::freg => types! { f: F16, F32; d: F64; },
            Self::vreg => &[],
        }
    }
}

pub(crate) fn is_e(target_features: &FxIndexSet<Symbol>) -> bool {
    target_features.contains(&sym::character('e'))
}

fn not_e(
    _arch: InlineAsmArch,
    _reloc_model: RelocModel,
    target_features: &FxIndexSet<Symbol>,
    _target: &Target,
    _is_clobber: bool,
) -> Result<(), &'static str> {
    if is_e(target_features) {
        Err("register can't be used with the `e` target feature")
    } else {
        Ok(())
    }
}

fn is_cheri(
    _arch: InlineAsmArch,
    _reloc_model: RelocModel,
    _target_features: &FxIndexSet<Symbol>,
    target: &Target,
    _is_clobber: bool,
) -> Result<(), &'static str> {
    if target.families.contains(&Cow::Borrowed("cheri")) {
        Ok(())
    } else {
        Err("register can only be used for a CHERI-enabled target")
    }
}

def_regs! {
    RiscV RiscVInlineAsmReg RiscVInlineAsmRegClass {
        x1: reg = ["x1", "ra"],
        x5: reg = ["x5", "t0"],
        x6: reg = ["x6", "t1"],
        x7: reg = ["x7", "t2"],
        x10: reg = ["x10", "a0"],
        x11: reg = ["x11", "a1"],
        x12: reg = ["x12", "a2"],
        x13: reg = ["x13", "a3"],
        x14: reg = ["x14", "a4"],
        x15: reg = ["x15", "a5"],
        x16: reg = ["x16", "a6"] % not_e,
        x17: reg = ["x17", "a7"] % not_e,
        x18: reg = ["x18", "s2"] % not_e,
        x19: reg = ["x19", "s3"] % not_e,
        x20: reg = ["x20", "s4"] % not_e,
        x21: reg = ["x21", "s5"] % not_e,
        x22: reg = ["x22", "s6"] % not_e,
        x23: reg = ["x23", "s7"] % not_e,
        x24: reg = ["x24", "s8"] % not_e,
        x25: reg = ["x25", "s9"] % not_e,
        x26: reg = ["x26", "s10"] % not_e,
        x27: reg = ["x27", "s11"] % not_e,
        x28: reg = ["x28", "t3"] % not_e,
        x29: reg = ["x29", "t4"] % not_e,
        x30: reg = ["x30", "t5"] % not_e,
        x31: reg = ["x31", "t6"] % not_e,
        c1: reg = ["c1", "cra"] % is_cheri,
        c5: reg = ["c5", "ct0"] % is_cheri,
        c6: reg = ["c6", "ct1"] % is_cheri,
        c7: reg = ["c7", "ct2"] % is_cheri,
        c10: reg = ["c10", "ca0"] % is_cheri,
        c11: reg = ["c11", "ca1"] % is_cheri,
        c12: reg = ["c12", "ca2"] % is_cheri,
        c13: reg = ["c13", "ca3"] % is_cheri,
        c14: reg = ["c14", "ca4"] % is_cheri,
        c15: reg = ["c15", "ca5"] % is_cheri,
        c16: reg = ["c16", "ca6"] % is_cheri,
        c17: reg = ["c17", "ca7"] % is_cheri,
        c18: reg = ["c18", "cs2"] % is_cheri,
        c19: reg = ["c19", "cs3"] % is_cheri,
        c20: reg = ["c20", "cs4"] % is_cheri,
        c21: reg = ["c21", "cs5"] % is_cheri,
        c22: reg = ["c22", "cs6"] % is_cheri,
        c23: reg = ["c23", "cs7"] % is_cheri,
        c24: reg = ["c24", "cs8"] % is_cheri,
        c25: reg = ["c25", "cs9"] % is_cheri,
        c26: reg = ["c26", "cs10"] % is_cheri,
        c27: reg = ["c27", "cs11"] % is_cheri,
        c28: reg = ["c28", "ct3"] % is_cheri,
        c29: reg = ["c29", "ct4"] % is_cheri,
        c30: reg = ["c30", "ct5"] % is_cheri,
        c31: reg = ["c31", "ct6"] % is_cheri,
        f0: freg = ["f0", "ft0"],
        f1: freg = ["f1", "ft1"],
        f2: freg = ["f2", "ft2"],
        f3: freg = ["f3", "ft3"],
        f4: freg = ["f4", "ft4"],
        f5: freg = ["f5", "ft5"],
        f6: freg = ["f6", "ft6"],
        f7: freg = ["f7", "ft7"],
        f8: freg = ["f8", "fs0"],
        f9: freg = ["f9", "fs1"],
        f10: freg = ["f10", "fa0"],
        f11: freg = ["f11", "fa1"],
        f12: freg = ["f12", "fa2"],
        f13: freg = ["f13", "fa3"],
        f14: freg = ["f14", "fa4"],
        f15: freg = ["f15", "fa5"],
        f16: freg = ["f16", "fa6"],
        f17: freg = ["f17", "fa7"],
        f18: freg = ["f18", "fs2"],
        f19: freg = ["f19", "fs3"],
        f20: freg = ["f20", "fs4"],
        f21: freg = ["f21", "fs5"],
        f22: freg = ["f22", "fs6"],
        f23: freg = ["f23", "fs7"],
        f24: freg = ["f24", "fs8"],
        f25: freg = ["f25", "fs9"],
        f26: freg = ["f26", "fs10"],
        f27: freg = ["f27", "fs11"],
        f28: freg = ["f28", "ft8"],
        f29: freg = ["f29", "ft9"],
        f30: freg = ["f30", "ft10"],
        f31: freg = ["f31", "ft11"],
        v0: vreg = ["v0"],
        v1: vreg = ["v1"],
        v2: vreg = ["v2"],
        v3: vreg = ["v3"],
        v4: vreg = ["v4"],
        v5: vreg = ["v5"],
        v6: vreg = ["v6"],
        v7: vreg = ["v7"],
        v8: vreg = ["v8"],
        v9: vreg = ["v9"],
        v10: vreg = ["v10"],
        v11: vreg = ["v11"],
        v12: vreg = ["v12"],
        v13: vreg = ["v13"],
        v14: vreg = ["v14"],
        v15: vreg = ["v15"],
        v16: vreg = ["v16"],
        v17: vreg = ["v17"],
        v18: vreg = ["v18"],
        v19: vreg = ["v19"],
        v20: vreg = ["v20"],
        v21: vreg = ["v21"],
        v22: vreg = ["v22"],
        v23: vreg = ["v23"],
        v24: vreg = ["v24"],
        v25: vreg = ["v25"],
        v26: vreg = ["v26"],
        v27: vreg = ["v27"],
        v28: vreg = ["v28"],
        v29: vreg = ["v29"],
        v30: vreg = ["v30"],
        v31: vreg = ["v31"],
        #error = ["x9", "s1", "c9", "cs1"] =>
            "s1 is used internally by LLVM and cannot be used as an operand for inline asm",
        #error = ["x8", "s0", "fp", "c8", "cs0"] =>
            "the frame pointer cannot be used as an operand for inline asm",
        #error = ["x2", "sp", "c2", "csp"] =>
            "the stack pointer cannot be used as an operand for inline asm",
        #error = ["x3", "gp", "c3", "cgp"] =>
            "the global pointer cannot be used as an operand for inline asm",
        #error = ["x4", "tp", "c4", "ctp"] =>
            "the thread pointer cannot be used as an operand for inline asm" ,
        #error = ["x0", "zero", "c0", "cnull"] =>
            "the zero register cannot be used as an operand for inline asm",
    }
}

impl RiscVInlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        out.write_str(self.name())
    }
}
