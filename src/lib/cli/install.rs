use crate::lib::{
    config::loader::load_cfg,
    fsutil::{copy::copy, paths::get_root},
    structs::{config::KelpDotConfig, pm::PackageManager},
    util::{exec::get_root_exec_program, os::{is_os,get_host_os}, pm::get_distro_pm, scripts::run_script}
};
use anyhow::Context;
use kelpdot_macros::*;
use std::{path::Path, process::Command};
pub fn install() -> anyhow::Result<()> {
    let root = get_root()?;
    cyan_print!("[INFO] Installing dotfiles {}", root);
    debug_print!("Building OS list...");
    let os = get_host_os()?;
    cyan_print!("Found OS {}", os.prettyname);
    let config: KelpDotConfig = load_cfg(root.clone())?;
    if let Some(scripts) = config.prerun {
        for script in scripts {
            cyan_print!("[PRERUN] Running script {}", script.path);
            run_script(root.clone(), script)?;
        }
    }
    if let Some(files) = config.homefiles {
        let home_files_path = format!("{}/home", root);
        for file in files {
            if let Some(distro) = &file.onlyon {
                if &os.name != distro && !os.submatches.iter().any(|mat| mat == distro) {
                    debug_print!("Not installing file {} because host != onlyon", file);
                    break;
                }
            }
            let home =
                std::env::var("HOME").with_context(|| red!("Unable to get env var $HOME!"))?; // Get $HOME path or crash
            debug_print!("Home: {}", home);
            if Path::new(&format!("{}/{}", home_files_path, file.path)).exists() {
                cyan_print!("[INFO] Installing {}", file);
                copy(
                    format!("{}/{}", home_files_path, file.path),
                    format!("{}/{}", home, file.path),
                )?;
            }
        }
    }
    // The work of rootfiles copy is **really** different:
    // Firstly we check if file exist
    // We create a Shell script with required files copies
    // We execute it as root
    // DONE!
    if let Some(files) = config.rootfiles {
        let mut bash_code = String::from("#!/usr/bin/env sh\n#This script has been auto-generated and will be runned by KelpDot\n#It isn't intended to be modified manually\n");
        for file in files {
            if let Some(distro) = &file.onlyon {
                if &os.name != distro && !os.submatches.iter().any(|mat| mat == distro) {
                    debug_print!("Not installing file {} because host != onlyon", file);
                    break;
                }
            }
            let fpath = format!("{}{}", root, file.path);
            // ShBang isn't really needed, I know
            let path = Path::new(&fpath);
            let dest_parent = Path::new(&file.path).parent().unwrap().to_str().unwrap();
            if path.exists() {
                bash_code = format!(
                    "{}if [[ ! -d {} ]]\nthen\nmkdir -p {}\nfi\ncp -r {} {}\n",
                    bash_code,
                    dest_parent,
                    dest_parent,
                    path.to_str().unwrap(),
                    dest_parent
                );
            }
        }
        std::fs::write("/tmp/kelpdot_install.sh", bash_code)?;
        let rexec = get_root_exec_program()?;
        Command::new(&rexec) // Use SH because some systems symlinks it to bash / zsh / ash
            .arg("sh")
            .arg("/tmp/kelpdot_install.sh")
            .status()
            .with_context(|| red!("Unable to call rootfiles install script!"))?;
    }
    // Check if we need to install packages
    if let Some(packages) = config.packages {
        let pm = get_distro_pm()?; 
        if let Some(gentoo) = packages.gentoo {
            if is_os("gentoo")? {
                if let Some(pkgs) = gentoo.packages {
                        pm.install_packages(pkgs)?;
                }
                if let Some(file) = gentoo.with_file {
                    let mut packages: Vec<String> = Vec::new();
                    let filepath = format!("{}/{}", root, file);
                    let fpath = Path::new(&filepath);
                    if fpath.exists() {
                        let contents = std::fs::read_to_string(filepath)?;
                        contents.lines().into_iter().for_each(|x| {
                            packages.push(String::from(x));
                        });
                    }
                        pm.install_packages(packages)?;
                }
            }
        }
    }
    if let Some(scripts) = config.postrun {
        for script in scripts {
            cyan_print!("[POSTRUN] Running script {}", script.path);
            run_script(root.clone(), script)?;
        }
    }
    Ok(())
}
