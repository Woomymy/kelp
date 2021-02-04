extern crate libkelp;
#[test]
pub fn external_root_works() {
    std::env::set_var("DOTFILES_ROOT", "/");
    assert_eq!(String::from("/"), libkelp::lib::fsutil::paths::get_root().unwrap());
}