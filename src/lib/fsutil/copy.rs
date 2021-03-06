use anyhow::{Context, Result};
use kelpdot_macros::*;
use std::{fs::read_dir, path::Path};
/// Copy a file / Directory
pub fn copy(source: String, dest: String) -> anyhow::Result<()> {
    let destpath = Path::new(&dest);
    let parent = destpath.parent().unwrap();
    debug_print!("Checking parents path...");
    if !parent.exists() {
        std::fs::create_dir_all(parent)
            .with_context(|| red!("Unable to create dir {}", parent.to_str().unwrap()))?;
    }
    if Path::new(&source).is_file() {
        std::fs::copy(source.clone(), dest.clone())
            .with_context(|| red!("Unable to copy {} to {}", source, dest))?;
    } else {
        copy_file_or_dir(&source, &dest)
            .with_context(|| red!("Unable to copy dir {} to {}", source, dest))?;
    }
    Ok(())
}
fn copy_file_or_dir(source: &str, dest: &str) -> anyhow::Result<()> {
    if Path::new(&dest).exists() {
        if Path::new(&dest).is_file() {
            std::fs::remove_file(dest)?;
        } else {
            std::fs::remove_dir_all(dest)?;
        }
    }
    std::fs::create_dir_all(&dest)?;
    copy_process(source, dest)?;
    Ok(())
}
fn copy_process(src: &str, dest: &str) -> Result<()> {
    let dir = read_dir(src).with_context(|| format!("\x1b[96mFailed to read {}\x1b[m", src))?;
    for item in dir {
        let entrypath = item?.path();
        if entrypath.is_file() {
            let destination = format!(
                "{}/{}",
                dest,
                entrypath.file_name().unwrap().to_str().unwrap()
            );
            std::fs::copy(entrypath, destination)?;
        } else {
            let fname = entrypath.file_name().unwrap().to_str().unwrap();
            let destination = format!("{}/{}", dest, fname);
            let source = format!("{}/{}", src, fname);
            copy_file_or_dir(&source, &destination)?;
        }
    }
    Ok(())
}
