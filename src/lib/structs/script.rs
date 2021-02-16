use serde::{Deserialize, Serialize};
#[derive(PartialEq, Debug, Serialize, Deserialize)]
/// A "prerun|postrun" script
pub struct Script {
    /// Path relative to $DOTFILES_ROOT
    pub path: String,
    /// If the script needs to start as root
    pub elevated: Option<bool>,
}
// To make sure we can use println!("{}", script)
impl std::fmt::Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}
