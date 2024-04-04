use std::{collections::HashMap, path::PathBuf};
use serde::Deserialize;

use super::io_task::IoTask;

#[derive(Debug, Deserialize)]
pub struct IoSheet {
    pub tasks_path: PathBuf,
    pub solutions_path: Option<PathBuf>,
    pub tasks: HashMap<String, IoTask>,
}