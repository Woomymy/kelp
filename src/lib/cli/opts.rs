use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "kelpdot")]
pub enum Cli {
    Save {},
    Init {},
    Install {},
}
