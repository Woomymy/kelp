mod lib;
use structopt::StructOpt;
use lib::cli::Cli;
fn main() -> anyhow::Result<()> {
    match Cli::from_args() {
        Cli::Save { } => {
            let root = std::env::var("DOTFILES_PATH").unwrap_or_else(|_| String::from("."));
            let confpath = format!("{}/kelp.yaml", root); 
            if !std::path::Path::new(&confpath).exists() {
                println!("{}", console::style("Please create a kelp.yml file with the \"kelp init\" command!").red().bold());
                std::process::exit(1);
            }
            let config = lib::config::load_config(confpath)?;
            println!("{:#?}", config);
        }
    }
    Ok(())
}