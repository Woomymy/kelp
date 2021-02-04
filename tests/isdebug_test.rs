extern crate libkelp;
#[test]
pub fn isdebug_true_works() {
    std::env::set_var("KELP_DEBUG", "true");
    assert_eq!(true, libkelp::lib::util::env::is_debug())
}
#[test]
pub fn isdebug_false_works() {
    std::env::set_var("KELP_DEBUG", "false");
    assert_eq!(false, libkelp::lib::util::env::is_debug())
}
