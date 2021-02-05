/// Prints a text in cyan
pub fn cyan(msg: &str) {
    println!("{}", console::style(msg).cyan().bold());
}
/// Prints a text in red
pub fn red(msg: &str) {
    println!("{}", console::style(msg).red().bold());
}
/// Prints a text in green
pub fn green(msg: &str) {
    println!("{}", console::style(msg).green().bold());
}
/// Prints a text in magenta
pub fn magenta(msg: &str) {
    println!("{}", console::style(msg).magenta().bold());
}
