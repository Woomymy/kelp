#[macro_use]
extern crate shadow_rs;
shadow!(build);
use structopt::StructOpt;

#[macro_use]
extern crate kelpdot_macros;

mod lib;
use lib::{cli::opts::Cli, terminal::messages::not_yet_implemented};
fn main() -> anyhow::Result<()> {
    green!("KelpDot V{}", build::PKG_VERSION);
    debug_print!("Commit {} Branch {}", build::SHORT_COMMIT, build::BRANCH);
    debug_print!("Built with {}-{}", build::RUST_VERSION, build::RUST_CHANNEL);
    debug_print!("Commit by {}", build::COMMIT_AUTHOR);
    green!("==============");
    // Check CLI options
    match Cli::from_args() {
        Cli::Save {} => {
            lib::cli::save::save()?;
        }
        Cli::Install {} => {
            not_yet_implemented();
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
