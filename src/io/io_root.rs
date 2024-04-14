use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::structs::module::Module;

use super::io_module::IoModule;

const CURRENT_IO_VERSION: &str = "1.0.0";

#[derive(Debug, Serialize, Deserialize)]
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

impl IoRoot {
    pub fn store(modules: &Vec<Module>, io_path: &PathBuf) {
        let io_root = IoRoot {
            schema_version: CURRENT_IO_VERSION.to_string(),
            modules: modules.iter()
                .map(|module| IoModule::from(module))
                .collect()
        };

        let serialized_data = serde_json5::to_string(&io_root)
            .unwrap_or_else(|_| panic!("Unable to serialize."));
        fs::write(io_path, serialized_data)
            .unwrap_or_else(|_| panic!("Unable to write to '{}'.", io_path.display()));
    }
}
