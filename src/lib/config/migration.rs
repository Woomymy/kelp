use crate::lib::structs::{
    config::KelpDotConfig,
    fileinfo::FileInfo,
    legacy::{LegacyFileInfo, LegacyKelpConfig},
};
pub fn migrate_configs(config: LegacyKelpConfig) -> anyhow::Result<KelpDotConfig> {
    let mut rfiles = vec![];
    for file in config.rootfiles {
        rfiles.push(fileinfo_to_new(file));
    }
    let mut homefiles = vec![];
    for file in config.homedir {
        homefiles.push(fileinfo_to_new(file));
    }
    let new = KelpDotConfig {
        rootfiles: Some(rfiles),
        homefiles: Some(homefiles),
        prerun: Some(vec![]),
        postrun: Some(vec![]),
    };
    Ok(new)
}
pub fn fileinfo_to_new(info: LegacyFileInfo) -> FileInfo {
    FileInfo {
        backuponly: info.backuponly,
        path: info.path,
        name: info.name,
        onlyon: info.onlyon,
    }
}
