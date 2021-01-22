/// Gets the root of dotfiles using DOTFILES_ROOT path or .
pub fn get_root() -> String {
    return std::env::var("DOTFILES_ROOT").unwrap_or_else(|_| String::from("."))
}