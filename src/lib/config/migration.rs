use crate::lib::structs::{
    config::KelpDotConfig,
    fileinfo::FileInfo,
    legacy::{LegacyFileInfo, LegacyKelpConfig},
};
/// Migrate v1.0.* config file to newer format
pub fn migrate_configs(config: LegacyKelpConfig) -> anyhow::Result<KelpDotConfig> {
    let mut rfiles = vec![];
    for file in config.rootfiles {
        rfiles.push(fileinfo_to_new(file));
    }
    let mut homefiles = vec![];
    for file in config.homedir {
        homefiles.push(fileinfo_to_new(file));
    }
    Ok(KelpDotConfig {
        rootfiles: Some(rfiles),
        homefiles: Some(homefiles),
        postsave: Some(vec![]),
        prerun: Some(vec![]),
        postrun: Some(vec![]),
        packages: None,
    })
}
/// Convert a fileinfo
pub fn fileinfo_to_new(info: LegacyFileInfo) -> FileInfo {
    FileInfo {
        backuponly: info.backuponly,
        path: info.path,
        name: info.name,
        onlyon: info.onlyon,
    }
}
