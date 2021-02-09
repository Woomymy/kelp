use structopt::StructOpt;

#[macro_use]
extern crate kelpdot_macros;

mod lib;
use lib::cli::opts::Cli;
fn main() -> anyhow::Result<()> {
    green!("KelpDot V{}", env!("CARGO_PKG_VERSION"));
    green!("==============");
    // Check CLI options
    match Cli::from_args() {
        Cli::Save {} => {
            lib::cli::save::save()?;
        }
        Cli::Install {} => {
            red!("Not yet implemeted!");
        }
        Cli::Init {} => {
            lib::cli::init::init()?;
        }
        Cli::Migrate {} => {
            lib::cli::migrate::migrate()?;
        }
    }
    Ok(())
}
