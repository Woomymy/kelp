use std::path::Path;
pub fn get_root_exec_program() -> anyhow::Result<String> {
    if Path::new("/usr/bin/doas").exists() {
        return Ok(String::from("doas"));
    }
    if Path::new("/usr/bin/sudo").exists() {
        return Ok(String::from("sudo"));
    }
    Err(anyhow::Error::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not find sudo or doas!",
    )))
}
