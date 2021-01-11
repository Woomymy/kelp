use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]// Required by serde_yaml
pub struct KelpConfig {
    /// The name of this configuration
    pub name: String
}