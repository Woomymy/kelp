use crate::lib::{
    config::autoconfig::{get_home_files, get_root_files},
    fsutil::paths::get_root,
    structs::config::KelpDotConfig,
};
use kelpdot_macros::*;
use std::path::Path;
/// Init and autconfig
pub fn init() -> anyhow::Result<()> {
    let root = get_root()?;
    debug_print!("Root: {}", root);
    if Path::new(&format!("{}/kelp.yaml", root)).exists() {
        red!("{}/kelp.yaml already exists!", root);
        std::process::exit(1);
    }
    let rootfiles = get_root_files()?;
    let homefiles = get_home_files()?;
    let cfg = KelpDotConfig {
        homefiles: Some(homefiles),
        rootfiles: Some(rootfiles),
        postrun: Some(vec![]),
        prerun: Some(vec![]),
        postsave: Some(vec![]),
    };
    let conf_path = format!("{}/kelp.yaml", root);
    magenta!("[INFO] Config file {} created!", conf_path);
    std::fs::write(conf_path, serde_yaml::to_string(&cfg)?)?;
    Ok(())
}
