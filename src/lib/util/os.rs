pub struct Os {
    /// Name of the os
    pub name: String,
    /// File used to reconise it
    pub file: String,
}
/// Find oses to detect
pub fn build_os_list(root: String) -> anyhow::Result<Vec<Os>> {
    let search_paths = vec!["/etc/kelp/os.yaml", "/etc/kelp/os.yml"];

    Ok(vec![])
}
