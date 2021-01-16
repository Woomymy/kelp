use super::structs::{FileInfo, KelpConfig};
/// Find files that can automaticaly copyied by Kelp
pub fn autoconfig(config: &mut KelpConfig) -> anyhow::Result<()> {
    let rawhome = include_str!("../../config/home.yaml");
    let home: Vec<FileInfo> = serde_yaml::from_str(&rawhome)?;
    for f in home {
        if std::path::Path::new(&format!(
            "{}/{}",
            std::env::var("HOME").unwrap_or_else(|_| String::from(".")),
            f.path
        ))
        .exists()
        {
            config.homedir.push(f);
        }
    }
    Ok(())
}
