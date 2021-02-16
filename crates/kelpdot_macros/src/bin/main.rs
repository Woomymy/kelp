#[macro_use]
extern crate kelpdot_macros;
fn main() {
    red!("Hello {}", "world");
    magenta!("Hello {}", "world");
    cyan!("Hello {}", "world");
    green!("Hello {}", "world");
    println!("Hello {}", "world");
}