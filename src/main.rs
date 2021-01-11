mod lib;
use structopt::StructOpt;
use lib::cli::Cli;
use std::path::Path;
fn main() -> anyhow::Result<()> {
    match Cli::from_args() {
        Cli::Save { } => {
            // Get the root of dotfiles
            let root = std::env::var("DOTFILES_PATH").unwrap_or_else(|_| String::from("."));
            let confpath = format!("{}/kelp.yaml", root); 
            if !Path::new(&confpath).exists() {
                println!("{}", console::style("Please create a kelp.yml file with the \"kelp init\" command!").red().bold());
                std::process::exit(1);
            }
            // Read and load configuration
            let config = lib::config::load_config(confpath)?;
            println!("{}", console::style(format!("Backing up configuration {}...", config.name)).cyan());
            let homedir_path = format!("{}/home", root);
            // Create home direcotry
            if Path::new(&homedir_path).exists() {
                std::fs::remove_dir_all(&homedir_path)?;
            }
            std::fs::create_dir_all(&homedir_path)?;
            for file in config.homedir {
                let filepath = format!("{}/{}", std::env::var("HOME").expect("Unable to find $HOME env var!"), file.path);
                if !Path::new(&filepath).exists() {
                    println!("{}", console::style(format!("File {} doesn't exist! Skipping...", filepath)).red());
                    break;
                }
                let path = Path::new(&filepath);
                let mut tomake
                    = path.parent().unwrap().to_str().unwrap();
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
                std::fs::create_dir_all(format!("{}/home/{}", root, tomake))?;
            }
        }
    }
    Ok(())
}