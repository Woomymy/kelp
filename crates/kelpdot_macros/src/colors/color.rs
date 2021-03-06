//#![macro_use]

#[macro_export]
/// Prints a text in cyan
macro_rules! cyan_print {
    ($($arg:tt)*) => {
        println!("{}", format!("\x1B[1;96m{}\x1b[0;m", format!($($arg)*)));
    };
}
#[macro_export]
/// Prints a text in red
macro_rules! red_print {
    ($($arg:tt)*) => {
        println!("{}", format!("\x1B[1;91m{}\x1b[0;m", format!($($arg)*)));
    };
}
#[macro_export]
/// Prints a text in green
macro_rules! green_print {
    ($($arg:tt)*) => {
        println!("{}", format!("\x1B[1;92m{}\x1b[0;m", format!($($arg)*)));
    };
}
#[macro_export]
/// Prints a text in magenta
macro_rules! magenta_print {
    ($($arg:tt)*) => {
        println!("{}", format!("\x1B[1;95m{}\x1b[0;m", format!($($arg)*)));
    };
}
#[macro_export]
macro_rules! red {
    ($($arg:tt)*) => {
        format!("\x1b[91m{}\x1b[m", format!($($arg)*));
    };
}