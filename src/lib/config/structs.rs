use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
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
pub struct KelpConfig {
    /// The name of the configuration
    pub name: String,
    /// Files to copy contained in /home/$USER
    pub homedir: Vec<FileInfo>,
    /// Root-located files ex: /var/lib/portage/world
    pub rootfiles: Vec<FileInfo>,
}
impl KelpConfig {
    /// Transforms configuration to a String
    pub fn to_string(&self) -> serde_yaml::Result<String> {
        return serde_yaml::to_string(self);
    }
}
