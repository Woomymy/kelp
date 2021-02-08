use kelpdot_macros::*;
use std::path::Path;
use crate::lib::structs::fileinfo::FileInfo;
pub fn get_root_files() -> anyhow::Result<Vec<FileInfo>> {
    let mut files: Vec<FileInfo> = Vec::new();
    let conf_bundled = include_str!("../../config/root.yaml");
    let config_yaml: Vec<FileInfo> = serde_yaml::from_str(conf_bundled)?;
    for file in config_yaml {
        if Path::new(&file.path).exists() {
            debug_print!("Add autoconf file {}", file.path);
            files.push(file);
        }
    }
    Ok(files)
}
pub fn get_home_files() -> anyhow::Result<Vec<FileInfo>> {
    let mut files: Vec<FileInfo> = Vec::new();
    let conf_bundled = include_str!("../../config/home.yaml");
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from(""));  // This doesn't work with "" but it won't panic
    let config_yaml: Vec<FileInfo> = serde_yaml::from_str(conf_bundled)?;
    for file in config_yaml {
        if Path::new(&format!("{}/{}", home, file.path)).exists() {
            debug_print!("Add autoconf file {}", file.path);
            files.push(file);
        }
    }
    Ok(files)
}