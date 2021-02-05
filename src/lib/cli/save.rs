use crate::lib::{
    config::loader::load_cfg,
    fsutil::paths::{get_root, get_to_make},
    terminal::colors::*,
    terminal::debug::debug_print,
    util::os::get_host_os,
};
use std::path::Path;
/// Backup dotfiles
pub fn save() -> anyhow::Result<()> {
    let root = get_root()?;
    cyan(&format!("[INFO] Saving dotfiles {}...", root));
    debug_print("Building OS list...");
    let os = get_host_os()?;
    cyan(&format!("[INFO] Found Os {}", os.prettyname));
    let config = load_cfg(root.clone())?;
    if let Some(files) = config.homefiles {
        let home = std::env::var("HOME")?;
        debug_print(&format!("Home: {}", home));
        if Path::new(&format!("{}/home", root)).exists() {
            std::fs::remove_dir_all(&format!("{}/home", root))?;
        }
        std::fs::create_dir(format!("{}/home", root))?;
        for f in files {
            green(&format!("[SAVE] Copying file {}...", f));
            let path = format!("{}/{}", home, f.path);
            let file = Path::new(&path);
            if !file.exists() {
                magenta(&format!(
                    "[WARNING] Skipping {} because it doesn't exist!",
                    f
                ));
                break;
            }
            let tomake = get_to_make(f.path)?;
        }
    }
    if let Some(files) = config.rootfiles {
        for f in files {
            // WIP
            green(&format!("[SAVE] Copying file {}", f));
            //let tomake = get_to_make(f.path)?;
        }
    }
    Ok(())
}
