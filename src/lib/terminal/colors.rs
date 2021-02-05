pub fn cyan(msg: &str) {
    println!("{}", console::style(msg).cyan());
}
pub fn red(msg: &str) {
    println!("{}", console::style(msg).red());
}
