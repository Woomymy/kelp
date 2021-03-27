use crate::lib::{structs::pm::PackageManager, util::exec::get_root_exec_program};
use anyhow::{Context, Result};
use kelpdot_macros::red;
use std::process::Command;
/// The gentoo's package manager
pub struct Portage {
    systempackages: Vec<String>,
}

impl PackageManager for Portage {
    fn install_package(&mut self, package: &str) -> Result<()> {
        if self.systempackages.is_empty() {
            let world = std::fs::read_to_string("/var/lib/portage/world")
                .with_context(|| red!("Can't read WORLD file!"))?;
            world.lines().into_iter().for_each(|x| {
                self.systempackages.push(x.to_string());
            });
        }
        if !self.systempackages.iter().any(|x| x == package) {
            let executor = get_root_exec_program()?;
            Command::new(&executor)
                .arg("emerge")
                .arg(package)
                .status()
                .with_context(|| red!("Unable to install {}!", package))?;
        }
        Ok(())
    }

    fn install_packages(&mut self, packages: Vec<String>) -> Result<()> {
        if !packages.is_empty() {
            if self.systempackages.is_empty() {
                let world = std::fs::read_to_string("/var/lib/portage/world")
                    .with_context(|| red!("Can't read WORLD file!"))?;
                world.lines().into_iter().for_each(|x| {
                    self.systempackages.push(String::from(x));
                });
            }
            let executor = get_root_exec_program()?;
            let mut cmd = Command::new(&executor);
            cmd.arg("emerge");
            for pack in packages {
                if !self.systempackages.iter().any(|x| *x == pack) {
                    cmd.arg(pack);
                }
            }
            cmd.status()?;
        }
        Ok(())
    }
    fn new() -> Self {
        Self {
            systempackages: vec![],
        }
    }
}
