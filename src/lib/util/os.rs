use std::path::Path;
/// Check the  /etc files to detect mother distro name
pub fn get_distro() -> anyhow::Result<String> {
    if check_path("/etc/gentoo-release") {
        return Ok(String::from("gentoo"));
    } else if check_path("/etc/arch-release") {
        return Ok(String::from("arch"));
    } else if check_path("/etc/alpine-release") {
        return Ok(String::from("alpine"));
    } else if check_path("/etc/debian_version") {
        return Ok(String::from("debian"));
    } else if check_path("/etc/redhat-release") {
        return Ok(String::from("redhat"));
    } else if check_path("/etc/SUSE-brand") {
        return Ok(String::from("suse"));
    }
    Err(anyhow!("Unable to dertermine OS!"))
}
fn check_path(path: &str) -> bool {
    Path::new(path).exists()
}
