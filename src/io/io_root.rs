use std::{fs, path::PathBuf};

use serde::Deserialize;

use super::io_module::IoModule;

#[derive(Debug, Deserialize)]
pub struct IoRoot {
    pub schema_version: String,
    pub modules: Vec<IoModule>,
}
impl From<&PathBuf> for IoRoot {
    fn from(path: &PathBuf) -> IoRoot {
        serde_json5::from_str(
            &fs::read_to_string(path)
                .unwrap_or_else(|_| panic!("Unable to read file '{}'", path.display())),
        )
        .unwrap_or_else(|_| panic!("Content of '{}' does not match schema.", path.display()))
    }
}
