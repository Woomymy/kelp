pub fn cyan(msg: &str) {
    println!("{}", console::style(msg).cyan());
}
pub fn red(msg: &str) {
    println!("{}", console::style(msg).red().bold());
}
pub fn green(msg: &str) {
    println!("{}", console::style(msg).green().bold());
}
pub fn magenta(msg: &str) {
    println!("{}", console::style(msg).magenta().bold());
}
