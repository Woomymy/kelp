use anyhow::Context;
use structopt::StructOpt;

#[macro_use]
extern crate kelpdot_macros;

mod lib;
use lib::cli::opts::Cli;
fn main() -> anyhow::Result<()> {
    green_print!("KelpDot V{}", env!("CARGO_PKG_VERSION"));
    green_print!("==============");
    // Check CLI options
    match Cli::from_args() {
        Cli::Save {} => {
            lib::cli::save::save().with_context(|| red!("Unable to save the dotfiles!"))?;
        }
        Cli::Install {} => {
            lib::cli::install::install().with_context(|| red!("Unable to install the dotfiles!"))?;
        }
        Cli::Init {} => {
            lib::cli::init::init().with_context(|| red!("Unable to init kelpdot!"))?;
        }
        Cli::Migrate {} => {
            lib::cli::migrate::migrate().with_context(|| red!("Unable to migrate configurations!"))?;
        }
    }
    Ok(())
}
