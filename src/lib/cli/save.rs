use crate::lib::{fsutil::paths::get_root, terminal::colors::*, terminal::debug::debug_print};
pub fn save() -> anyhow::Result<()> {
    let root = get_root()?;
    cyan(&format!("[INFO] Saving dotfiles {}...", root));
    debug_print("Building OS list...");
    crate::lib::util::os::build_os_list(root)?;
    Ok(())
}
