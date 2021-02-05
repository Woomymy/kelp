use structopt::StructOpt;
#[derive(StructOpt)]
#[structopt(name = "kelpdot", about = "Simple dotfiles manager with YAML configuration and usefull features.")]
pub enum Cli {
    /// Backup dotfiles
    Save {},
    /// Initialise kelp with autoconfiguration (NYI)
    Init {},
    /// Install all dotfiles (NYI)
    Install {},
    /// Migrate V1.0.X config to newer config format
    Migrate {},
}
