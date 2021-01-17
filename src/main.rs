mod lib;
use lib::cli::Cli;
use std::path::Path;
#[macro_use]
extern crate anyhow;
use structopt::StructOpt;
fn main() -> anyhow::Result<()> {
    match Cli::from_args() {
        Cli::Save {} => {
            // Get the root of dotfiles
            let root = std::env::var("DOTFILES_PATH").unwrap_or_else(|_| String::from("."));
            let confpath = format!("{}/kelp.yaml", root);
            if !Path::new(&confpath).exists() {
                println!(
                    "{}",
                    console::style("Please create a kelp.yml file with the \"kelp init\" command!")
                        .red()
                        .bold()
                );
                std::process::exit(1);
            }
            // Read and load configuration
            let config = lib::config::load_config(confpath)?;
            println!(
                "{}",
                console::style(format!("Backing up configuration {}...", config.name)).cyan()
            );
            // The directory where /home/$USER files are located in backup
            let homedir_path = format!("{}/home", root);
            // Create home direcotry
            if Path::new(&homedir_path).exists() {
                // If home directory exists, remove it
                std::fs::remove_dir_all(&homedir_path)?;
            }
            std::fs::create_dir_all(&homedir_path)?;
            // Backup everyone $HOME file
            for file in config.homedir {
                let distro = lib::util::get_distro()?;
                if let Some(host) = file.onlyon {
                    if distro != host {
                        println!(
                            "{}",
                            console::style(format!(
                                "Not saving {} because it can only be saved on {}",
                                file.path, host
                            ))
                            .bold()
                            .cyan()
                        );
                        continue;
                    }
                }
                let filepath = format!(
                    "{}/{}",
                    std::env::var("HOME").expect("Unable to find $HOME env var!"),
                    file.path
                );
                if !Path::new(&filepath).exists() {
                    println!(
                        "{}",
                        console::style(format!("File {} doesn't exist! Skipping...", filepath))
                            .red()
                    );
                    continue;
                }
                if let Some(fname) = file.name {
                    println!(
                        "{}",
                        console::style(format!("Copying {}...", fname))
                            .bold()
                            .magenta()
                    );
                } else {
                    println!(
                        "{}",
                        console::style(format!("Copying {}...", file.path))
                            .bold()
                            .magenta()
                    );
                }
                let path = Path::new(&filepath);
                let mut tomake = path.parent().unwrap().to_str().unwrap();
                let start;
                // Make sure paths are splitted corectly:
                // If you are logged in as root (uid 0)
                // Your $HOME path is /root/
                // Then start must be set to 2
                // If you are logged in as a normal user (others uid)
                // Your $HOME path is /home/$USERNAME
                // Then start must be set to 3
                if users::get_current_uid() == 0 {
                    start = 2;
                } else {
                    start = 3;
                }
                let splitted: Vec<&str> = tomake.split('/').collect();
                let pure = splitted[start..splitted.len()].join("/");
                tomake = &pure;
                let spath: Vec<&str> = path.to_str().unwrap().split('/').collect();
                let fname;
                // If directory name doesn't ends with /, only one element to remove
                if path.is_file() || !file.path.ends_with('/') {
                    fname = spath[spath.len() - 1];
                } else {
                    fname = spath[spath.len() - 2];
                }
                std::fs::create_dir_all(format!("{}/home/{}", root, tomake))?;
                if path.is_file() {
                    std::fs::copy(
                        path.to_str().unwrap(),
                        format!("{}/home/{}/{}", root, tomake, fname),
                    )?;
                } else {
                    let dest = format!("{}/home/{}/{}", root, tomake, fname);
                    copy_dir::copy_dir(path, dest)?;
                }
            }
            for file in config.rootfiles {
                let host = lib::util::get_distro()?;
                if let Some(h) = file.onlyon {
                    if h != host {
                        println!(
                            "{}",
                            console::style(format!(
                                "Not saving {} because it can only be saved on {}",
                                file.path, host
                            ))
                            .bold()
                            .cyan()
                        );
                        continue;
                    }
                }
                if !Path::new(&file.path).exists() {
                    println!(
                        "{}",
                        console::style(format!(
                            "Skipping file {} because it doesn't exist!",
                            file.path
                        ))
                        .red()
                        .bold()
                    );
                    continue;
                }
                let splittedpath: Vec<&str> = file.path.split('/').collect();
                let end;
                let path = Path::new(&file.path);
                if path.is_file() || !file.path.ends_with('/') {
                    end = 1;
                } else {
                    end = 2;
                }
                let pure = splittedpath[0..splittedpath.len() - end].join("/");
                println!(
                    "{}",
                    console::style(format!("Copying {}", file.path))
                        .bold()
                        .magenta()
                );
                if !Path::new(&format!("{}/{}", root, pure)).exists() {
                    std::fs::create_dir_all(format!("{}/{}", root, pure))?;
                }
                if path.is_file() {
                    let dest =
                        format!("{}/{}/{}", root, pure, splittedpath[splittedpath.len() - 1]);
                    std::fs::copy(&file.path, dest)?;
                } else {
                    let end;
                    if !path.ends_with("/") || path.is_file() {
                        end = 2;
                    } else {
                        end = 1;
                    }
                    if Path::new(&format!(
                        "{}/{}/{}",
                        root,
                        pure,
                        splittedpath[splittedpath.len() - end]
                    )).exists() {
                        std::fs::remove_dir_all(format!(
                            "{}/{}/{}",
                            root,
                            pure,
                            splittedpath[splittedpath.len() - end]
                        ))?;
                    }
                    copy_dir::copy_dir(&file.path, format!(
                        "{}/{}/{}",
                        root,
                        pure,
                        splittedpath[splittedpath.len() - end]
                    ))?;
                }
            }
        }
        Cli::Init {} => {
            let root = std::env::var("DOTFILES_PATH").unwrap_or_else(|_| String::from("."));
            if !Path::new(&root).exists() {
                println!(
                    "{}",
                    console::style(format!("Path {} doesn't exist!", root))
                        .red()
                        .bold()
                );
                std::process::exit(1);
            }
            if Path::new(&format!("{}/kelp.yaml", root)).exists() {
                println!(
                    "{}",
                    console::style(format!("File {}/kelp.yaml already exists!", root))
                        .red()
                        .bold()
                );
                std::process::exit(1);
            }
            // Create a basic configuration
            let mut config = lib::config::KelpConfig {
                name: String::from("Dotfiles"),
                homedir: Vec::new(),
                rootfiles: Vec::new(),
            };
            lib::config::autoconfig(&mut config)?;
            println!("{}", console::style("Autoconfiguration applied!").yellow());
            std::fs::write(format!("{}/kelp.yaml", root), config.to_string()?)?;
        }
        Cli::Install {} => {
            let root = std::env::var("DOTFILES_PATH").unwrap_or_else(|_| String::from("."));
            let confpath = format!("{}/kelp.yaml", root);
            if !Path::new(&confpath).exists() {
                println!(
                    "{}",
                    console::style("Please create a kelp.yml file with the \"kelp init\" command!")
                        .red()
                        .bold()
                );
                std::process::exit(1);
            }
            // Read and load configuration
            let config = lib::config::load_config(confpath)?;
            let home = std::env::var("HOME")?;
            for file in config.homedir {
                let filepath = format!("{}/home/{}", root, file.path);
                if !Path::new(&filepath).exists() {
                    println!(
                        "{}",
                        console::style(format!("Skipping {}", file)).bold().yellow()
                    );
                    continue;
                }
                let host = lib::util::get_distro()?;
                if let Some(ref distro) = file.onlyon {
                    if distro != &host {
                        println!(
                            "{}",
                            console::style(format!(
                                "Skipping {} because it can only be installed on {}",
                                file, &distro
                            ))
                            .bold()
                            .yellow()
                        );
                        continue;
                    }
                }
                let path = Path::new(&filepath);
                let dest: Vec<&str> = file.path.split('/').collect();
                let pure = dest[0..dest.len() - 1].join("/");
                if !Path::new(&format!("{}/{}", home, pure)).exists() {
                    std::fs::create_dir_all(format!("{}/{}", home, pure))?;
                }
                let destination;
                if pure == "" {
                    destination = format!("{}/{}", home, dest[dest.len() - 1])
                } else {
                    destination = format!("{}/{}/{}", home, pure, dest[dest.len() - 1])
                }
                println!(
                    "{}",
                    console::style(format!("Copying {} to {}", filepath, destination))
                        .cyan()
                        .bold()
                );
                if path.is_file() {
                    std::fs::copy(filepath, destination)?;
                } else {
                    if Path::new(&destination).exists() {
                        std::fs::remove_dir_all(&destination)?;
                    }
                    copy_dir::copy_dir(filepath, destination)?;
                }
            }
            let mut bashcode = String::from("#!/usr/bin/env bash"); // Add shbang
            for file in config.rootfiles {
                if let Some(f) = file.backuponly {
                    if f == true {
                        println!(
                            "{}",
                            console::style(format!("Skipping {} because it is backup only!", file))
                                .bold()
                                .yellow()
                        );
                        continue;
                    }
                }
                let filepath = format!("{}{}", root, file.path);
                if !Path::new(&filepath).exists() {
                    println!(
                        "{}",
                        console::style(format!("Skipping {}", file)).bold().yellow()
                    );
                    continue;
                }
                let host = lib::util::get_distro()?;
                if let Some(ref distro) = file.onlyon {
                    if distro != &host {
                        println!(
                            "{}",
                            console::style(format!(
                                "Skipping {} because it can only be installed on {}",
                                file, &distro
                            ))
                            .bold()
                            .yellow()
                        );
                        continue;
                    }
                }
                println!(
                    "{}",
                    console::style(format!("Installing {} in {}", file, filepath))
                        .bold()
                        .yellow()
                );
                if Path::new(&filepath).is_file() {
                    bashcode = format!(
                        "\n{}\nif [[ -f {} ]]\nthen\ncp -r {} {}\nfi\n",
                        bashcode, filepath, filepath, file.path
                    );
                } else {
                    let dest: Vec<&str> = file.path.split('/').collect();
                    let end;
                    if file.path.ends_with("/") {
                        end = 2
                    } else {
                        end = 1
                    }
                    let pure = dest[0..dest.len() - end].join("/");
                    bashcode = format!(
                        "\n{}\nif [[ -d {} ]]\nthen\ncp -r {} {}\nfi\n",
                        bashcode, filepath, filepath, pure
                    );
                }
            }
            std::fs::write("/tmp/kelp_sh_install_root.sh", bashcode)?;
            let mut cmd = runas::Command::new("bash");
            cmd.arg("/tmp/kelp_sh_install_root.sh");
            match cmd.status() {
                Ok(_) => {}
                Err(_) => {
                    println!("{}", console::style("Unable to install root files!").red().bold());
                }
            }
        }
    }
    Ok(())
}
