use std::collections::HashMap;
use serde::Deserialize;

use super::io_sheet::IoSheet;

#[derive(Debug, Deserialize)]
pub struct IoModule {
    pub sheets: HashMap<String, IoSheet>,
}