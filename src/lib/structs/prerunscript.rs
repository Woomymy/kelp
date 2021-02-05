use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
/// A "prerun" script
pub struct PrerunScript {
    /// Path relative to $DOTFILES_ROOT
    pub path: String,
    /// If the script needs to start as root
    pub elevated: Option<bool>,
}

impl std::fmt::Display for PrerunScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}
