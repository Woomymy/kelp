use std::path::Path;
use kelpdot_macros::*;
use crate::lib::{fsutil::{paths::get_root,copy::copy},util::os::get_host_os,config::loader::load_cfg,structs::config::KelpDotConfig};
pub fn install() -> anyhow::Result<()> {
    let root = get_root()?;
    cyan!("[INFO] Installing dotfiles {}", root);
    debug_print!("Building OS list...");
    let os = get_host_os()?;
    cyan!("Found OS {}", os.prettyname);
    let config: KelpDotConfig = load_cfg(root.clone())?;
    if let Some(files) = config.homefiles {
        let home_files_path = format!("{}/home", root);
        for file in files {
            let home = std::env::var("HOME")?; // Get $HOME path or crash
            debug_print!("Home: {}", home);
            if Path::new(&format!("{}/{}", home_files_path, file.path)).exists() {
                cyan!("[INFO] Installing {}", file);
                copy(format!("{}/{}", home_files_path, file.path), format!("{}/{}", home, file.path))?;
            }
        }
    }
    Ok(())
}