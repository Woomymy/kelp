/// Prints a text in red
pub fn print_red(msg: String) {
    println!("{}", console::style(msg).red().bold());
}
