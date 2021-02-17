use kelpdot_macros::*;
use std::path::Path;
/// Copy a file / Directory
pub fn copy(source: String, dest: String) -> anyhow::Result<()> {
    let destpath = Path::new(&dest);
    let parent = destpath.parent().unwrap();
    debug_print!("Checking parents path...");
    if !parent.exists() {
        std::fs::create_dir_all(parent)?;
    }
    if destpath.exists() {
        if destpath.is_file() {
            std::fs::remove_file(destpath)?;
        } else {
            std::fs::remove_dir_all(destpath)?;
        }
    }
    if Path::new(&source).is_file() {
        std::fs::copy(source, dest)?;
    } else {
        copy_dir::copy_dir(source, dest)?;
    }
    Ok(())
}
