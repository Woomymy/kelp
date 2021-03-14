use anyhow::Result;
/// Base trait for PM integrations
trait PackageManager {
    fn install_packages(packages: Vec<String>) -> Result<()>;
}
