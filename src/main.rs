#[macro_use]
extern crate shadow_rs;
shadow!(build);
use structopt::StructOpt;
mod lib;
use lib::{cli::opts::Cli, terminal::{messages::not_yet_implemented, debug::debug_print, colors::*}};
fn main() -> anyhow::Result<()> {
    green(&format!("KelpDot V{}-{}", build::PKG_VERSION, build::BRANCH));
    debug_print(&format!("Commit {} Branch {}", build::SHORT_COMMIT, build::BRANCH));
    debug_print(&format!("Built with {}-{}", build::RUST_VERSION, build::RUST_CHANNEL));
    debug_print(&format!("Commit by {}", build::COMMIT_AUTHOR));
    green("==============");
    // Check CLI options
    match Cli::from_args() {
        Cli::Save {} => {
            lib::cli::save::save()?;
        }
        Cli::Install {} => {
            not_yet_implemented();
        }
        Cli::Init {} => {
            not_yet_implemented();
        }
        Cli::Migrate {} => {
            lib::cli::migrate::migrate()?;
        }
    }
    Ok(())
}
