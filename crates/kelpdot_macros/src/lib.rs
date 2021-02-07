#![macro_use]

pub extern crate console;

#[macro_export]
/// Prints a text in cyan
macro_rules! cyan {
    ($($arg:tt)*) => {
        println!("{}", ::kelpdot_macros::console::style(format!($($arg)*)).cyan().bold());
    };
}
#[macro_export]
/// Prints a text in red
macro_rules! red {
    ($($arg:tt)*) => {
        println!("{}", ::kelpdot_macros::console::style(format!($($arg)*)).red().bold());
    };
}
#[macro_export]
/// Prints a text in green
macro_rules! green {
    ($($arg:tt)*) => {
        println!("{}", ::kelpdot_macros::console::style(format!($($arg)*)).green().bold());
    };
}
#[macro_export]
/// Prints a text in magenta
macro_rules! magenta {
    ($($arg:tt)*) => {
        println!("{}", ::kelpdot_macros::console::style(format!($($arg)*)).magenta().bold());
    };
}
