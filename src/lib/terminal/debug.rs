pub fn debug_print(msg: &str) {
    let var = std::env::var("KELP_DEBUG").unwrap_or_else(|_| String::from(""));
    if var == "true" {
        println!("{}", console::style(&format!("[DEBUG] {}", msg)).yellow());
    }
}
