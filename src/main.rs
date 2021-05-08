#[macro_use]
extern crate kelpdot_macros;
mod lib;
fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        usage(args);
        std::process::exit(1);
    }
    match args[1].as_str() {
        "install" => {
            lib::cli::install::install()?;
        }
        "init" => {
            lib::cli::init::init()?;
        }
        "save" => {
            lib::cli::save::save()?;
        }
        _ => {
            usage(args);
            std::process::exit(1);
        }
    }
    Ok(())
}

pub fn usage(args: Vec<String>) {
    magenta_print!("KelpDot V{}", env!("CARGO_PKG_VERSION"));
    red_print!("Invalid usage! {}", args.join(" "));
    green_print!("Options:");
    cyan_print!("{} save : Save the dotfiles", args[0]);
    cyan_print!("{} install : Install the dotfiles", args[0]);
    cyan_print!("{} init : Initialise Kelpdot", args[0]);
}
