use anyhow::Result;
use super::KelpConfig;
pub fn load_config(path: String) -> Result<KelpConfig> {
    let fcontent = std::fs::read_to_string(path)?;
    let config: KelpConfig = serde_yaml::from_str(&fcontent)?;
    Ok(config) 
}