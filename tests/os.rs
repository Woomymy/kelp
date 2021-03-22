extern crate libkelp;
use libkelp::lib::util::os::{is_os,get_host_os};
// Tests for is_os
#[test]
fn os_test() {
    let os = get_host_os().unwrap();
    assert!(is_os(&os.name).unwrap());
}
