#![macro_use]

#[macro_export]
/// Prints a text in cyan
macro_rules! cyan {
    ($($arg:tt)*) => {
        println!("{}", format!("\x1B[1;96m{}\x1b[0;m", format!($($arg)*)));
    };
}
#[macro_export]
/// Prints a text in red
macro_rules! red {
    ($($arg:tt)*) => {
        println!("{}", format!("\x1B[1;91m{}\x1b[0;m", format!($($arg)*)));
    };
}
#[macro_export]
/// Prints a text in green
macro_rules! green {
    ($($arg:tt)*) => {
        println!("{}", format!("\x1B[1;92m{}\x1b[0;m", format!($($arg)*)));
    };
}
#[macro_export]
/// Prints a text in magenta
macro_rules! magenta {
    ($($arg:tt)*) => {
        println!("{}", format!("\x1B[1;95m{}\x1b[0;m", format!($($arg)*)));
    };
}
