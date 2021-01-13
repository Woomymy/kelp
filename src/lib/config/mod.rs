mod loader;
pub use loader::*;
mod structs;
pub use structs::*;
mod autoconfig;
pub use autoconfig::*;
// Use cli::* to make sure imports not look like this:
// lib::cli::cli::Cli;
// Now it's more like this:
// lib::cli::Cli
