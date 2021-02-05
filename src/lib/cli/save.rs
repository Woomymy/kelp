use crate::lib::{
    fsutil::paths::get_root, terminal::colors::*, terminal::debug::debug_print,
    util::os::get_host_os,
    config::loader::load_cfg,
};
/// Backup dotfiles
pub fn save() -> anyhow::Result<()> {
    let root = get_root()?;
    cyan(&format!("[INFO] Saving dotfiles {}...", root));
    debug_print("Building OS list...");
    let os = get_host_os()?;
    cyan(&format!("[INFO] Found Os {}", os.prettyname));
    let _config = load_cfg(root)?;
    Ok(())
}
