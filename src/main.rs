use structopt::StructOpt;
mod lib;
use lib::cli::Cli;
fn main() {
    match Cli::from_args() {
        Cli::Save { } => {
            println!("Not yet implemented!");
        }
        Cli::Install { } => {
            println!("Not yet implemented!");
        }
        Cli::Init { } => {
            println!("Not yet implemented!");
        }
    }
}