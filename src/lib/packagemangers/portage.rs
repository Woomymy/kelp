use std::process::Command;
use anyhow::{Result,Context};
use crate::lib::{
    structs::pm::PackageManager,
    util::exec::get_root_exec_program,
};
use kelpdot_macros::red;
/// The gentoo's package manager
pub struct Portage {
    name: String, 
}

impl PackageManager for Portage {
    fn install_package(&self, package: &str) -> Result<()> {
        let executor = get_root_exec_program()?;
        Command::new(&executor)
        .arg("emerge")
        .arg(package)
        .status()
        .with_context(|| red!("Unable to install {} with {}", package, self.name))?;
        Ok(())
    }
    fn install_packages(&self, packages: Vec<String>) -> Result<()> {
        Ok(())
    }
    fn new() -> Self {
        Self {
            name: String::from("portage")
        }
    }
}

