use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LegacyFileInfo {
    /// Name used to call the file: ex: I3 configuration
    pub name: Option<String>,
    /// The path to the file / directory (relative to /home/$USER)ex: .config/i3/config
    pub path: String,
    /// Only install if distro is ?
    pub onlyon: Option<String>,
    /// If the file shouldn't be installed, mark is as "backuponly"
    pub backuponly: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)] // Required by serde_yaml
pub struct LegacyKelpConfig {
    /// The name of the configuration
    pub name: String,
    /// Files to copy contained in /home/$USER
    pub homedir: Vec<LegacyFileInfo>,
    /// Root-located files ex: /var/lib/portage/world
    pub rootfiles: Vec<LegacyFileInfo>,
}
