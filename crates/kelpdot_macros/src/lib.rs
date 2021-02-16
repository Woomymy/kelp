pub mod colors;
pub mod debug;
#[test]
pub fn isdebug_true_works() {
    std::env::set_var("KELP_DEBUG", "true");
    assert_eq!(true, debug::dbg::is_debug())
}
#[test]
pub fn isdebug_false_works() {
    std::env::set_var("KELP_DEBUG", "false");
    assert_eq!(false, debug::dbg::is_debug())
}
