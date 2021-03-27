extern crate libkelp;
use libkelp::lib::util::os::{get_host_os, is_os};
// Tests for is_os
#[test]
fn os_test() {
    let os = get_host_os().unwrap();
    assert!(is_os(&os.name).unwrap());
}
