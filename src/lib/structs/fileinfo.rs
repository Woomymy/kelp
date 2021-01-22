pub use serde::{
    Serialize, Deserialize
}
#[derive(Serialize, Deserialize)]
/// FileINFO struct: used for represent a file to backup
pub struct FileInfo {
    /// The name of the file, optional but recommended to reconize files
    pub name: Option<String>
    /// The path to the file "relative to / or /home/$USER"
    pub path: String,
    /// Is the file backup only ? Or must it be reinstalled
    pub backuponly: Option<bool>,
    /// If the file is only usable on specific disto, specify it: 
    /// Examples
    /// gentoo: Gentoo based distro
    /// arch: Arch based distros
    /// debian: Debian based distro
    /// redhat: Redhat based distro (redhat included!)
    /// alpine: Alpine linux based distro
    pub onlyon: Option<String>
}
impl std::fmt::Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(n) = &self.name {
            write!(f, "{}", n)
        } else {
            write!(f, "{}", self.path)
        }
    }
}