use anyhow::Result;
/// Base trait for PM integrations
trait PackageManager {
    fn install_packages(packages: Vec<String>) -> Result<()>;
    fn install_package(package: &str) -> Result<()>;
}
/// Packages managers such as NPM or cargo
trait ExternalPackageManager: PackageManager {
    fn install_on_distro<T: PackageManager>(pm: T) -> Result<()>;
}

