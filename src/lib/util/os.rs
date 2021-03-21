use anyhow::{Context, Result};
use kelpdot_macros::*;
use serde::{Deserialize, Serialize};
use std::path::Path;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Os {
    /// Name of the os
    pub name: String,
    /// File used to reconise it
    pub file: String,
    /// "Priority" of the OS
    /// For example
    /// Arch has pritority 1 and file /etc/arch-release
    /// And manjaro has priority 2 and files /etc/arch-release AND /etc/manjaro-release
    /// If both /etc/manjaro-release and /etc/arch-release exists, the higher priority will be used
    /// In this case it's 2 (manjaro)
    pub priority: i16,
    /// The "pretty" name of the OS: for example "Arch GNU/Linux"
    pub prettyname: String,
    /// Submatches (parent distros)
    pub submatches: Vec<String>,
}
/// Find oses to detect
pub fn build_os_list() -> Result<Vec<Os>> {
    let mut osyaml: Vec<Os> = vec![];
    let search_paths = vec!["/etc/kelpdot/os.yaml", "/etc/kelpdot/os.yml"];
    for path in search_paths {
        if Path::new(path).exists() {
            debug_print!("Reading OS file {}", path);
            let content = std::fs::read_to_string(path)
                .with_context(|| red!("Unable to read OS file {}", path))?;
            let yaml: Vec<Os> = serde_yaml::from_str(&content)
                .with_context(|| red!("Failed to parse OS file {}", path))?;
            for os in yaml {
                osyaml.push(os);
            }
        }
    }
    debug_print!("Reading bundled OS file...");
    let bundled = include_str!("../../config/oses.yaml");
    let oses: Vec<Os> = serde_yaml::from_str(&bundled).with_context(|| {
        red!("Unable to parse bundled OS list file (please report this with a GitHub issue)")
    })?;
    for os in oses {
        osyaml.push(os);
    }
    Ok(osyaml)
}
/// Gets the host's OS
pub fn get_host_os() -> anyhow::Result<Os> {
    // Get a Vec<Os> wich is a list of reconisables oses
    let oses = build_os_list()?;
    let mut validoses: Vec<Os> = vec![];
    for system in oses {
        if Path::new(&system.file).exists() && system.name != "generic" {
            validoses.push(system);
        }
    }
    let mut sys: Os = Os {
        name: String::from("generic"),
        file: String::from("/"),
        priority: 0,
        prettyname: String::from("Generic GNU/Linux OS"),
        submatches: vec![],
    };
    for system in validoses {
        if sys.priority < system.priority {
            sys = system;
        }
    }
    Ok(sys)
}

pub fn is_os(name: &str) -> Result<bool> {
    let os = get_host_os()?;
    if os.submatches.iter().any(|x| x == name) || os.name == name {
        return Ok(true);
    }
    Ok(false)
}
