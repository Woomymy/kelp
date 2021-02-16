use std::path::Path;
/// Gets the root of dotfiles using DOTFILES_ROOT path or .
pub fn get_root() -> anyhow::Result<String> {
    let basepath = std::env::var("DOTFILES_ROOT").unwrap_or_else(|_| String::from("."));
    let full = std::fs::canonicalize(basepath)?;
    Ok(full.to_str().unwrap().to_string())
}
/// Get name of directory to make
pub fn get_to_make(path: String) -> anyhow::Result<String> {
    let home = std::env::var("HOME")?;
    // If file is located at /home
    if Path::new(&format!("{}/{}", home, path)).exists() {
        Ok(Path::new(&path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned())
    } else {
        Ok(Path::new(&path)
            .parent()
            .unwrap()
            .strip_prefix("/")? // Remove / to get path like etc/config instead of /etc/config
            .to_str()
            .unwrap()
            .to_owned())
    }
}
