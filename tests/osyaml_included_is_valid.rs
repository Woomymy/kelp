extern crate libkelp;
#[test]
pub fn osyaml_is_ok() {
    let stri = include_str!("../src/config/oses.yaml");
    match serde_yaml::from_str::<Vec<libkelp::lib::util::os::Os>>(&stri) {
        Ok(_) => {}
        Err(_) => {
            panic!("Invalid YAML!");
        }
    }
}