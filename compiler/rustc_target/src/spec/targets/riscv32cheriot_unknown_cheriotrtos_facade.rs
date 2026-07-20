//! This target is only meant to be used internally for running tests, which
//! require some level of standard library support.

use crate::spec::{Env, Target};

pub(crate) fn target() -> Target {
    let mut target = super::riscv32cheriot_unknown_cheriotrtos::target();

    // Use "sim" environment so to differentiate from our target proper
    // without needing to rename the OS.
    target.env = Env::Sim;
    // Testing infrastructure expects this.
    target.executables = true;
    // Avoid any confusion.
    target.entry_name = "rust_main".into();
    // Set it as singlethread for now.
    target.singlethread = true;

    target
}
