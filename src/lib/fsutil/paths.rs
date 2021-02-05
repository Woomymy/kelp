use std::path::Path;
/// Gets the root of dotfiles using DOTFILES_ROOT path or .
pub fn get_root() -> anyhow::Result<String> {
    let basepath = std::env::var("DOTFILES_ROOT").unwrap_or_else(|_| String::from("."));
    let full = std::fs::canonicalize(basepath)?;
    Ok(full.to_str().unwrap().to_string())
}
pub fn get_to_make(path: String) -> anyhow::Result<String> {
    let home = std::env::var("HOME")?;
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
            .strip_prefix("/")?
            .to_str()
            .unwrap()
            .to_owned())
    }
}
