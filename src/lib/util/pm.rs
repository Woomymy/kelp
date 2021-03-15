use crate::lib::util::os;
use crate::lib::structs::pm::PackageManager;
use crate::lib::packagemangers::portage::Portage;
pub fn get_distro_pm() -> anyhow::Result<impl PackageManager> {
    let os = os::get_host_os()?;
    if os.submatches.iter().any(|x| x == "gentoo") || os.name == "gentoo" {
        return Ok(Portage::new())
    }
    Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Can not find OS packagemanger!")))
}
