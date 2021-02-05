use structopt::StructOpt;
// TOOD: Add CLI documentation
#[derive(StructOpt)]
#[structopt(name = "kelpdot")]
pub enum Cli {
    Save {},
    Init {},
    Install {},
    Migrate {},
}
