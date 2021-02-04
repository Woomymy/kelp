use structopt::StructOpt;
mod lib;
use lib::{cli::opts::Cli, terminal};
fn main() -> anyhow::Result<()> {
    // Check CLI options
    match Cli::from_args() {
        Cli::Save {} => {
            lib::cli::save::save()?;
        }
        Cli::Install {} => {
            terminal::messages::not_yet_implemented();
        }
        Cli::Init {} => {
            terminal::messages::not_yet_implemented();
        }
    }
    Ok(())
}
