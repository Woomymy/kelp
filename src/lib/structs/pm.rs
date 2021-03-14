use anyhow::Result;
/// Base trait for PM integrations
trait PackageManager {
    fn install_packages(packages: Vec<String>) -> Result<()>;
    fn install_packages(package: &str) -> Result<()>;
}
/// Packages managers such as NPM or cargo
trait ExternalPackageManager: PackageManger {
    fn install_on_distro<T: PackageManger>(pm: T) -> Result<()>;
}

