extern crate libkelp;
#[test]
pub fn osyaml_is_ok() {
    let stri = include_str!("../src/config/oses.yaml");
    match serde_yaml::from_str::<Vec<libkelp::lib::util::os::Os>>(&stri) {
        Ok(_) => {}
        Err(_) => {
            panic!("Invalid OS YAML!");
        }
    }
}
#[test]
pub fn root_autoconfig_yaml_is_ok() {
    let stri = include_str!("../src/config/root.yaml");
    match serde_yaml::from_str::<Vec<libkelp::lib::structs::fileinfo::FileInfo>>(&stri) {
        Ok(_) => {}
        Err(_) => {
            panic!("Invalid ROOT autoconfig YAML!");
        }
    }
}
#[test]
pub fn home_autoconfig_yaml_is_ok() {
    let stri = include_str!("../src/config/home.yaml");
    match serde_yaml::from_str::<Vec<libkelp::lib::structs::fileinfo::FileInfo>>(&stri) {
        Ok(_) => {}
        Err(_) => {
            panic!("Invalid HOME autoconfig YAML!");
        }
    }
}
