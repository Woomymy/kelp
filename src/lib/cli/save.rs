use crate::lib::fsutil::paths::get_root;
pub fn save() -> anyhow::Result<()> {
    let root = get_root()?;
    println!("{}", root);
    Ok(())
}
