use crate::lib::util::env::is_debug;
pub fn debug_print(msg: &str) {
    if is_debug() {
        println!("{}", console::style(&format!("[DEBUG] {}", msg)).yellow());
    }
}
