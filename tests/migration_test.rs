extern crate libkelp;
use libkelp::lib::structs::*;
#[test]
pub fn test_config_migration_empty() {
    let old = libkelp::lib::structs::legacy::LegacyKelpConfig {
        name: String::from("Hello"),
        homedir: vec![],
        rootfiles: vec![],
    };
    assert_eq!(
        libkelp::lib::config::migration::migrate_configs(old).unwrap(),
        libkelp::lib::structs::config::KelpDotConfig {
            homefiles: Some(vec![]),
            rootfiles: Some(vec![]),
            postsave: Some(vec![]),
            prerun: Some(vec![]),
            postrun: Some(vec![])
        }
    );
}
#[test]
pub fn test_config_migration_with_values() {
    let old = libkelp::lib::structs::legacy::LegacyKelpConfig {
        name: String::from("Hello"),
        homedir: vec![legacy::LegacyFileInfo {
            name: Some(String::from("Hello")),
            path: String::from("/etc/hellow"),
            backuponly: Some(true),
            onlyon: Some(String::from("gentoo")),
        }],
        rootfiles: vec![],
    };
    assert_eq!(
        libkelp::lib::config::migration::migrate_configs(old).unwrap(),
        libkelp::lib::structs::config::KelpDotConfig {
            homefiles: Some(vec![fileinfo::FileInfo {
                name: Some(String::from("Hello")),
                path: String::from("/etc/hellow"),
                backuponly: Some(true),
                onlyon: Some(String::from("gentoo")),
            }]),
            rootfiles: Some(vec![]),
            postsave: Some(vec![]),
            prerun: Some(vec![]),
            postrun: Some(vec![])
        }
    );
}
