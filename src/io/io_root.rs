use serde::Deserialize;

use super::io_module::IoModule;

#[derive(Debug, Deserialize)]
pub struct IoRoot {
    pub schema_version: String,
    pub modules: Vec<IoModule>
}