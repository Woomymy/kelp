use std::path::Path;
use crate::lib::{
    structs::{
        legacy::LegacyKelpConfig
    },
    fsutil::paths::get_root,
    terminal::colors::*,
    config::migration::migrate_configs,
};
pub fn migrate() -> anyhow::Result<()> {
    let root = get_root()?;
    cyan(&format!("[INFO] Migrating dotfiles {}", root));
    if !Path::new(&format!("{}/kelp.yaml", root)).exists() {
        red(&format!("[ERROR] {}/kelp.yaml doesn't exist!", root));
        std::process::exit(1);
    }
    let config: LegacyKelpConfig = serde_yaml::from_str(&std::fs::read_to_string(format!("{}/kelp.yaml",root))?)?;
    let new = migrate_configs(config)?;
    std::fs::write(format!("{}/kelp.yaml", root), serde_yaml::to_string(&new)?)?;
    Ok(())
}