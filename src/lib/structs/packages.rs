use serde::{Deserialize, Serialize};
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    pub with_file: Option<String>,
    pub packages: Option<Vec<String>>,
}
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Packages {
    pub gentoo: Option<PackageInfo>,
    pub arch: Option<PackageInfo>,
    pub debian: Option<PackageInfo>,
    pub fedora: Option<PackageInfo>,
    pub npm: Option<PackageInfo>,
    pub gems: Option<PackageInfo>,
}
