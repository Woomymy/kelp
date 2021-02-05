use crate::lib::structs::{
    fileinfo::FileInfo, postrunscript::PostrunScript, prerunscript::PrerunScript,
};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
/// This is the new Configuration struct
pub struct KelpDotConfig {
    /// Files stored in $HOME/*
    pub homefiles: Option<Vec<FileInfo>>,
    /// Files relative to /
    pub rootfiles: Option<Vec<FileInfo>>,
    /// Scripts to run BEFORE installing the dotfiles.
    /// Path relative to $DOTFILES_ROOT
    pub prerun: Option<Vec<PrerunScript>>,
    /// Scripts to run After installing the dotfiles
    /// Path relative to $DOTFILES_ROOT
    pub postrun: Option<Vec<PostrunScript>>,
}
