use std::path::Path;
/// Copy a file / Directory
pub fn copy(source: String, dest: String) -> anyhow::Result<()> {
    if Path::new(&source).is_file() {
        std::fs::copy(source, dest)?;
    } else {
        copy_dir::copy_dir(source, dest)?;
    }
    Ok(())
}