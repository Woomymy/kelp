use anyhow::Result;
/// Base trait for PM integrations
pub trait PackageManager {
    fn install_packages(&mut self, packages: Vec<String>) -> Result<()>;
    fn install_package(&mut self, package: &str) -> Result<()>;
    fn new() -> Self;
}
/// Packages managers such as NPM or cargo
pub trait ExternalPackageManager: PackageManager {
    fn install_on_distro<T: PackageManager>(&mut self, pm: T) -> Result<()>;
}
