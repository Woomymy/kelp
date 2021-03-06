#[macro_use]
extern crate kelpdot_macros;
fn main() {
    red_print!("Hello {}", "world");
    magenta_print!("Hello {}", "world");
    cyan_print!("Hello {}", "world");
    green_print!("Hello {}", "world");
    println!("Hello {}", "world");
}
