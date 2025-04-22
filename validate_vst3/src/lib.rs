//! # validate_vst3
//!
//! The whole point of this crate is providing glue code to run the
//! [VST3 SDK](https://github.com/steinbergmedia/vst3sdk)'s validator using `cargo test`.
//!
//! To be able to execute `cargo test`, you need to ensure that these prerequisites are met:
//!
//!   - The rismidi plugin bundles must be built (using `cargo xtask bundle`)
//!   - You must have a C++ compiler and [CMake](https://cmake.org) > 3.23 installed
//!
//! To ensure that `cargo test` still works out of the box (i.e, without `cargo xtask` calls) for
//! the parent workspace, this crate is explicitly excluded from it.

use color_eyre::eyre::{bail, eyre, Result};
use std::{path::Path, process::Command};

/// Validation modes that the VST3 validator offers
pub enum ValidationMode {
    /// Regular checks for all plugins
    Basic,

    /// Extensive tests (`-e`)
    Extensive,

    /// Only a single test suite (`-suite`)
    SingleSuite(String),
}

fn validator_cmd() -> Command {
    Command::new(env!("RISMIDI_VST3_VALIDATOR"))
}

/// Runs the validator for a given plugin.
pub fn validate_plugin(plugin: &Path, mode: &ValidationMode) -> Result<()> {
    let plugin_path = plugin
        .to_str()
        .ok_or(eyre!("Could render plugin path at {}", plugin.display()))?;

    let mut cmd = validator_cmd();

    match mode {
        ValidationMode::Extensive => {
            cmd.arg("-e");
        }
        ValidationMode::SingleSuite(suite) => {
            cmd.args(["-suite", suite]);
        }
        _ => {}
    };

    cmd.arg(plugin_path);

    let validation_result = cmd.output()?;
    if !validation_result.status.success() {
        let stdout = String::from_utf8_lossy(&validation_result.stdout);
        bail!("Validation of '{plugin_path}' failed:\n\n{stdout}");
    }

    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/testcases.rs"));
