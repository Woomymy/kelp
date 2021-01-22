use structopt::StructOpt;
mod lib;
use lib::cli::Cli;
use lib::terminal;
fn main() {
    match Cli::from_args() {
        Cli::Save {} => {
            terminal::messages::not_yet_implemented();
        }
        Cli::Install {} => {
            terminal::messages::not_yet_implemented();
        }
        Cli::Init {} => {
            terminal::messages::not_yet_implemented();
        }
    }
}
