use crate::lib::structs::config::KelpDotConfig;
use anyhow::Context;
use kelpdot_macros::*;
use std::path::Path;
/// Loads config
pub fn load_cfg(root: String) -> anyhow::Result<KelpDotConfig> {
    if !Path::new(&format!("{}/kelp.yaml", root)).exists() {
        red_print!("File {}/kelp.yaml not found!", root);
        std::process::exit(1);
    }
    debug_print!("Loading config {}/kelp.yaml", root);
    let cfg: KelpDotConfig = serde_yaml::from_str(
        &std::fs::read_to_string(format!("{}/kelp.yaml", root))
            .with_context(|| red!("Unable to read config file!"))?,
    )
    .with_context(|| red!("Unable to parse config file!"))?;
    Ok(cfg)
}
