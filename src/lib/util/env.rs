pub fn is_debug() -> bool {
    let var = std::env::var("KELP_DEBUG").unwrap_or_else(|_| String::from(""));
    if var == "true" || var == "yes" {
        return true;
    }
    false
}
