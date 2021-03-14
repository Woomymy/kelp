use anyhow::Context;

#[macro_use]
extern crate kelpdot_macros;
use clap::{crate_authors, App, SubCommand};
mod lib;
use lib::structs::pm::PackageManager;
use lib::packagemangers::portage::Portage;
fn main() -> anyhow::Result<()> {
    green_print!("KelpDot V{}", env!("CARGO_PKG_VERSION"));
    green_print!("==============");
    let kelp = App::new("KelpDot")
        .version(env!("CARGO_PKG_VERSION"))
        .author(crate_authors!())
        .about("A simple dotfiles manager written in Rust")
        .subcommands(vec![
            SubCommand::with_name("init")
                .about("Setups kelpdot in the current folder or in $DOTFILES_ROOT"),
            SubCommand::with_name("save").about("Saves the dotfiles"),
            SubCommand::with_name("install").about("Installs the dotfiles"),
            SubCommand::with_name("migrate").about("Migrates to newer config versions"),
        ])
        .get_matches();
    // Check CLI options
    if let Some(_m) = kelp.subcommand_matches("save") {
        lib::cli::save::save().with_context(|| red!("Unable to save the dotfiles!"))?;
    } else if let Some(_m) = kelp.subcommand_matches("install") {
        lib::cli::install::install().with_context(|| red!("Unable to install the dotfiles!"))?;
    } else if let Some(_m) = kelp.subcommand_matches("init") {
        lib::cli::init::init().with_context(|| red!("Unable to init kelpdot!"))?;
    } else if let Some(_m) = kelp.subcommand_matches("migrate") {
        lib::cli::migrate::migrate().with_context(|| red!("Unable to migrate configurations!"))?;
    } else {
        println!("{}", kelp.usage());
    }
    Ok(())
}
