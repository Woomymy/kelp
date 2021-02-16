#![macro_use]

pub fn is_debug() -> bool {
    let var = std::env::var("KELP_DEBUG").unwrap_or_else(|_| String::from(""));
    if var == "true" || var == "yes" {
        return true;
    }
    false
}

#[macro_export]
/// Print debug info
macro_rules! debug_print {
    ($($arg:tt)*) => {
        if ::kelpdot_macros::debug::dbg::is_debug() {
            println!("{}", ::kelpdot_macros::colors::console::style(format!("[DEBUG] {}", format!($($arg)*))).yellow().bold());
        }
    };
}