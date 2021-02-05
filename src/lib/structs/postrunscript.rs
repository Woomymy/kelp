/// A script that runs **AFTER** dotfiles install
pub struct PostrunScript {
    /// Path relative to $DOTFILES_ROOT
    pub path: String,
    /// If the script needs to start as root
    pub elevated: Option<bool>,
}
impl std::fmt::Display for PostrunScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}
