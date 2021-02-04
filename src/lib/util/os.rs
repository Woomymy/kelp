use std::path::Path;
use serde::{Deserialize, Serialize};
#[derive(Debug,Serialize,Deserialize)]
pub struct Os {
    /// Name of the os
    pub name: String,
    /// File used to reconise it
    pub file: String,
}
/// Find oses to detect
pub fn build_os_list(root: String) -> anyhow::Result<Vec<Os>> {
    let mut osyaml: Vec<Os> = vec!();
    let search_paths = vec!["/etc/kelpdot/os.yaml", "/etc/kelpdot/os.yml"];
    for path in search_paths {
        if Path::new(path).exists() {
            let content = std::fs::read_to_string(path)?;
            let yaml: Vec<Os> = serde_yaml::from_str(&content)?;
            for os in yaml {
                osyaml.push(os);
            }
        }
    }
    let bundled = include_str!("../../config/oses.yaml");
    let oses: Vec<Os> = serde_yaml::from_str(&bundled)?;
    for os in oses {
        osyaml.push(os);
    }
    Ok(osyaml)
}
