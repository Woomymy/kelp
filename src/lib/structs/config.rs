use crate::lib::structs::{fileinfo::FileInfo, script::Script};
use serde::{Deserialize, Serialize};
#[derive(PartialEq, Serialize, Deserialize, Debug)]
/// This is the new Configuration struct
pub struct KelpDotConfig {
    /// Files stored in $HOME/*
    pub homefiles: Option<Vec<FileInfo>>,
    /// Files relative to /
    pub rootfiles: Option<Vec<FileInfo>>,
    /// Scripts to run AFTER saving the dotfiles
    pub postsave: Option<Vec<Script>>,
    /// Scripts to run BEFORE installing the dotfiles.
    /// Path relative to $DOTFILES_ROOT
    pub prerun: Option<Vec<Script>>,
    /// Scripts to run After installing the dotfiles
    /// Path relative to $DOTFILES_ROOT
    pub postrun: Option<Vec<Script>>,
}
