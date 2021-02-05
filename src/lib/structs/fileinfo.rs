pub use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
/// FileINFO struct: used for represent a file to backup
pub struct FileInfo {
    /// The name of the file, optional but recommended to reconize files
    pub name: Option<String>,
    /// The path to the file "relative to / or /home/$USER"
    pub path: String,
    /// Is the file backup only ? Or must it be reinstalled
    pub backuponly: Option<bool>,
    /// If the file is only usable on specific disto, specify it
    /// Note: There's a list of bundled OSes
    pub onlyon: Option<String>,
}
// If the file is named, print it's name
// Else print it's path
impl std::fmt::Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(n) = &self.name {
            write!(f, "{}", n)
        } else {
            write!(f, "{}", self.path)
        }
    }
}
