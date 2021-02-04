use crate::lib::fsutil::paths::get_root;
use crate::lib::terminal::colors::*;
pub fn save() -> anyhow::Result<()> {
    let root = get_root()?;
    cyan(&format!("[INFO] Saving dotfiles {}...", root));
    crate::lib::util::os::build_os_list(root)?;
    Ok(())
}
