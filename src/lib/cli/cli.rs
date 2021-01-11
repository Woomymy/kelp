use structopt::StructOpt;
#[derive(StructOpt)]
#[structopt(name = "kelp")]
pub enum Cli {
    Save {},
}
