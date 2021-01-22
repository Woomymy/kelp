/// Gets the root of dotfiles using DOTFILES_ROOT path or .
pub fn get_root() -> anyhow::Result<String> {
    let basepath = std::env::var("DOTFILES_ROOT").unwrap_or_else(|_| String::from("."));
    let full = std::fs::canonicalize(basepath)?;
    Ok(format!("{}", full.to_str().unwrap()))
}
