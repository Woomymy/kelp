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
            println!("{}", format!("\x1B[1;93m{}\x1b[0;m", format!("[DEBUG] {}", format!($($arg)*))));
        }
    };
}