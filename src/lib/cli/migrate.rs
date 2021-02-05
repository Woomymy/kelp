use crate::lib::{
    config::migration::migrate_configs, fsutil::paths::get_root, structs::legacy::LegacyKelpConfig,
    terminal::colors::*,
};
use std::path::Path;
/// Migrates config from legacy v1.0.* config style
pub fn migrate() -> anyhow::Result<()> {
    let root = get_root()?;
    cyan(&format!("[INFO] Migrating dotfiles {}", root));
    // Make sure that kelp is initalised
    if !Path::new(&format!("{}/kelp.yaml", root)).exists() {
        red(&format!("[ERROR] {}/kelp.yaml doesn't exist!", root));
        std::process::exit(1);
    }
    // TODO: Try to load config with newer to make sure config isn't already migrated
    let config: LegacyKelpConfig =
        serde_yaml::from_str(&std::fs::read_to_string(format!("{}/kelp.yaml", root))?)?;
    let new = migrate_configs(config)?; // Migrate config
                                        // Write it to filesystem
    std::fs::write(format!("{}/kelp.yaml", root), serde_yaml::to_string(&new)?)?;
    Ok(())
}
