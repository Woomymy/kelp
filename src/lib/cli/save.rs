use crate::lib::{
    config::loader::load_cfg,
    fsutil::{
        copy::copy,
        paths::{get_root, get_to_make},
    },
    terminal::{
        colors::{cyan, green, magenta},
        debug::debug_print,
    },
    util::os::get_host_os,
};
use std::path::Path;
/// Backup dotfiles
pub fn save() -> anyhow::Result<()> {
    let root = get_root()?;
    cyan(&format!("[INFO] Saving dotfiles {}...", root));
    debug_print("Building OS list...");
    let os = get_host_os()?; // Here we get guest os; If OS is unreconized, return a generic GNU / Linux System
    cyan(&format!("[INFO] Found Os {}", os.prettyname));
    let config = load_cfg(root.clone())?; // Load a KelpConfig struct wich is basically $DOTFILES_ROOT/kelp.yaml
    if let Some(files) = config.homefiles {
        // If config has "homefiles" keys, copy each $HOME/$FILE
        let home = std::env::var("HOME")?; // Get $HOME path or crash
        debug_print(&format!("Home: {}", home));

        // Make sur that $DOTFILES_ROOT/home doesn't exist
        // or doesn't contain files
        if Path::new(&format!("{}/home", root)).exists() {
            std::fs::remove_dir_all(&format!("{}/home", root))?;
        }
        std::fs::create_dir(format!("{}/home", root))?;
        for f in files {
            green(&format!("[SAVE] Copying file {}...", f));
            let path = format!("{}/{}", home, f.path);
            let file = Path::new(&path);
            // Make sur that file exists
            if !file.exists() {
                magenta(&format!(
                    "[WARNING] Skipping {} because it doesn't exist!",
                    f
                ));
                break;
            }
            // Get path to make
            // Example:
            // home/**.config/i3** directory
            let tomake = get_to_make(f.path)?;
            // Create the file
            std::fs::create_dir_all(format!("{}/home/{}", root, tomake))?;
            copy(
                path.clone(),
                format!(
                    "{}/home/{}/{}",
                    root,
                    tomake,
                    file.file_name().unwrap().to_str().unwrap().to_owned()
                ),
            )?;
        }
        cyan(&format!("[OK] Homefiles saved!"));
    }
    // If config has "rootfiles" key, backup every file
    if let Some(files) = config.rootfiles {
        for f in files {
            // WIP
            green(&format!("[SAVE] Copying file {}", f));
            // Get path to make:
            // Example:
            // $DOTFILES_ROOT/etc/portage/repos.conf
            // let tomake = get_to_make(f.path)?;
        }
    }
    Ok(())
}
