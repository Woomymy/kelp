use crate::lib::{
    structs::config::KelpDotConfig,
    terminal::{colors::red, debug::debug_print},
};
use std::path::Path;
pub fn load_cfg(root: String) -> anyhow::Result<KelpDotConfig> {
    if !Path::new(&format!("{}/kelp.yaml", root)).exists() {
        red(&format!("File {}/kelp.yaml not found!", root));
        std::process::exit(1);
    }
    debug_print(&format!("Loading config {}/kelp.yaml", root));
    let cfg: KelpDotConfig =
        serde_yaml::from_str(&std::fs::read_to_string(format!("{}/kelp.yaml", root))?)?;
    Ok(cfg)
}
