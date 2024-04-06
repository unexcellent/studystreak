use std::{collections::HashMap, fs, path::PathBuf};

use serde::Deserialize;

use super::io_module::IoModule;

#[derive(Debug, Deserialize)]
pub struct IoRoot {
    pub schema_version: String,
    pub modules: HashMap<String, IoModule>
}
impl IoRoot {
    pub fn from(path: &PathBuf) -> IoRoot {
        serde_json5::from_str(
            &fs::read_to_string(path).expect(&format!("Unable to read file '{}'", path.display()))
        ).expect(&format!("Content of '{}' does not match schema.", path.display()))
    }
}