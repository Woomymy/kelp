use crate::lib::{
    config::migration::migrate_configs,
    fsutil::paths::get_root,
    structs::{config::KelpDotConfig, legacy::LegacyKelpConfig},
};
use kelpdot_macros::*;
use std::path::Path;
/// Migrates config from legacy v1.0.* config style
pub fn migrate() -> anyhow::Result<()> {
    let root = get_root()?;
    cyan_print!("[INFO] Migrating dotfiles {}", root);
    // Make sure that kelp is initalised
    if !Path::new(&format!("{}/kelp.yaml", root)).exists() {
        red_print!("[ERROR] {}/kelp.yaml doesn't exist!", root);
        std::process::exit(1);
    }
    let contents = std::fs::read_to_string(format!("{}/kelp.yaml", root))?;
    if serde_yaml::from_str::<KelpDotConfig>(&contents).is_ok() {
        green_print!("[MIGRATION] Config is already up-to-date");
        std::process::exit(0);
        // Exit with 0 because this isn't realy an "error",
        // if config is Up-to-date, the result is the same than migrating
    }
    let config: LegacyKelpConfig = serde_yaml::from_str(&contents)?;
    let new = migrate_configs(config)?; // Migrate config
                                        // Write it to filesystem
    std::fs::write(format!("{}/kelp.yaml", root), serde_yaml::to_string(&new)?)?;
    Ok(())
}
