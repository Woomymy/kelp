use crate::lib::{structs::pm::PackageManager, util::exec::get_root_exec_program};
use anyhow::{Context, Result};
use kelpdot_macros::red;
use std::process::Command;
/// The gentoo's package manager
pub struct Portage {}

impl PackageManager for Portage {
    fn install_package(&self, package: &str) -> Result<()> {
        let executor = get_root_exec_program()?;
        Command::new(&executor)
            .arg("emerge")
            .arg(package)
            .status()
            .with_context(|| red!("Unable to install {}!", package))?;
        Ok(())
    }
    fn install_packages(&self, packages: Vec<String>) -> Result<()> {
        let executor = get_root_exec_program()?;
        let mut cmd = Command::new(&executor);
        cmd.arg("emerge");
        for pack in packages {
            cmd.arg(pack);
        }
        cmd.status()?;
        Ok(())
    }
    fn new() -> Self {
        Self {}
    }
}
