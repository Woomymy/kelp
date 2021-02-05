extern crate libkelp;
use libkelp::lib::structs::*;
#[test]
pub fn test_config_migration_empty() {
    let old = libkelp::lib::structs::legacy::LegacyKelpConfig {
        name: String::from("Hello"),
        homedir: vec![],
        rootfiles: vec![]
    };
    assert_eq!(libkelp::lib::config::migration::migrate_configs(old).unwrap(), libkelp::lib::structs::config::KelpDotConfig {
        homefiles: Some(vec![]),
        rootfiles: Some(vec![]),
        postsave: Some(vec![]),
        prerun: Some(vec![]),
        postrun: Some(vec![])
    });
}
