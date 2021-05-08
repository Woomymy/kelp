use anyhow::Context;
use kelpdot_macros::red;
use std::path::Path;
/// Check if a env var is set or return default
pub fn get_path_from_env<T: ToString>(var: &str, def: T) -> anyhow::Result<String> {
    let mut base = std::env::var(var).unwrap_or_else(|_| def.to_string());
    if !Path::new(&base).exists() {
        base = def.to_string();
    }
    let full = std::fs::canonicalize(base)?;
    Ok(full.to_str().unwrap().to_string())
}
#[cfg(test)]
mod tests {
    #[test]
    fn get_path_fenv_test() {
        use super::get_path_from_env;
        println!("{}", std::env::current_dir().unwrap().to_str().unwrap().to_string());
        assert_eq!(std::env::current_dir().unwrap().to_str().unwrap().to_string(), get_path_from_env("SOME_USELESS_VAR", ".").unwrap())
    }
}
/// Gets the root of dotfiles using DOTFILES_ROOT path or .
pub fn get_root() -> anyhow::Result<String> {
    get_path_from_env("DOTFILES_ROOT", ".")
}
/// Gets the INSTALL ROOT
pub fn get_ins_root() -> anyhow::Result<String> {
    get_path_from_env("KELPDOT_INS_ROOT", "/")
}
/// Get name of directory to make
pub fn get_to_make(path: String) -> anyhow::Result<String> {
    let home = std::env::var("HOME").with_context(|| red!("Unable to get env var $HOME"))?;
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
