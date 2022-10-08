use color_eyre::eyre::{eyre, Context, Result};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    provide_validator();
    generate_testcases()?;
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}

fn provide_validator() {
    static VALIDATOR_ENV_VAR: &str = "RISMIDI_VST3_VALIDATOR";

    let validator_path = match std::env::var(VALIDATOR_ENV_VAR) {
        Ok(path) => {
            // the validator path is set from the outside => no need to build it ourselves
            path
        }
        Err(_) => {
            // no validator is set from the outside => let's build one
            let out_path = cmake::build("sdk_wrapper");
            format!("{}/bin/validator", out_path.display())
        }
    };

    println!("cargo:rerun-if-env-changed={VALIDATOR_ENV_VAR}");
    println!("cargo:rustc-env={VALIDATOR_ENV_VAR}={validator_path}");
}

fn generate_testcases() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("testcases.rs");

    let plugins = list_plugins()?;
    let plugin_dir = workspace_root()?.join("target").join("bundled");

    fs::write(&dest_path, generate_testcode(&plugins, &plugin_dir)).unwrap();

    Ok(())
}

fn generate_testcode(plugins: &[String], plugin_dir: &Path) -> String {
    let mut code = String::new();
    code += "\
#[cfg(test)]
mod validate {
    use crate::{validate_plugin, ValidationMode};
    use std::path::Path;
";

    for plugin in plugins {
        let plugin_path = plugin_dir.join(&format!("{plugin}.vst3")).into_os_string();
        let plugin_path = plugin_path.to_str().unwrap();
        code += &format!(
            "
    #[test]
    fn {plugin}_basic() {{
        let plugin_path = Path::new(r\"{plugin_path}\");
        validate_plugin(&plugin_path, &ValidationMode::Basic).unwrap();
    }}

    #[test]
    fn {plugin}_extensive() {{
        let plugin_path = Path::new(r\"{plugin_path}\");
        validate_plugin(&plugin_path, &ValidationMode::Extensive).unwrap();
    }}
"
        );
    }

    code += "}";

    code
}

fn workspace_root() -> Result<PathBuf> {
    let manifest_dir =
        env::var_os("CARGO_MANIFEST_DIR").ok_or(eyre!("Could not get cargo manifest directory"))?;
    let workspace_root = Path::new(&manifest_dir).parent().ok_or(eyre!(
        "'{}' has no parent directory",
        Path::new(&manifest_dir).display()
    ))?;
    Ok(workspace_root.to_path_buf())
}

fn list_plugins() -> Result<Vec<String>> {
    let bundler_config_file = workspace_root()?.join("bundler.toml").as_path().to_owned();
    println!(
        "cargo:rerun-if-changed={}",
        bundler_config_file.to_str().unwrap()
    );

    let config_string = fs::read_to_string(&bundler_config_file)
        .with_context(|| format!("Could not read '{}'", bundler_config_file.display()))?;
    let config: toml::Value = toml::from_str(&config_string)
        .with_context(|| format!("Could not parse '{}'", bundler_config_file.display()))?;

    let plugins: Vec<String> = match config.as_table() {
        Some(map) => map.keys().map(|s| s.to_owned()).collect(),
        None => vec![],
    };

    Ok(plugins)
}
