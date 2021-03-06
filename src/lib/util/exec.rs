use std::path::Path;
use kelpdot_macros::red;
use anyhow::{Error, Result};
pub fn get_root_exec_program() -> Result<String> {
    if Path::new("/usr/bin/doas").exists() {
        return Ok(String::from("doas"));
    }
    if Path::new("/usr/bin/sudo").exists() {
        return Ok(String::from("sudo"));
    }
    Err(Error::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        red!("Could not find sudo or doas!"),
    )))
}
