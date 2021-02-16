use std::{path::Path,process::Command};
use kelpdot_macros::*;
use crate::lib::{fsutil::{paths::get_root,copy::copy},util::{os::get_host_os,exec::get_root_exec_program},config::loader::load_cfg,structs::config::KelpDotConfig};
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
    // The work of rootfiles copy is **really** different:
    // Firstly we check if file exist
    // We create a Shell script with required files copies
    // We execute it as root
    // DONE!
    if let Some(files) = config.rootfiles {
        for file in files {
            let fpath = format!("{}{}", root, file.path);
            let mut bash_code = String::from("#!/usr/bin/env bash\n#This script has been auto-generated and will be runned by KelpDot\n#It isn't intended to be modified manually\n");
            let path = Path::new(&fpath);
            let dest_parent = Path::new(&file.path).parent().unwrap().to_str().unwrap();
            if path.exists() {
                bash_code = format!("{}if [[ ! -d {} ]]\nthen\nmkdir -p {}\nfi\ncp -r {} {}\n", bash_code, dest_parent, dest_parent, path.to_str().unwrap(), file.path);
            }
            std::fs::write("/tmp/kelpdot_install.sh", bash_code)?;
        }
        let rexec = get_root_exec_program()?;
        Command::new(&rexec) // Use SH because some systems symlinks it to bash / zsh / ash
                        .arg("sh")
                        .arg("/tmp/kelpdot_install.sh")
                        .status()?;
        
    }
    Ok(())
}