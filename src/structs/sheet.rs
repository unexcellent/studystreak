use std::{collections::HashMap, path::PathBuf};

use super::task::Task;

/// An exercise sheet (a single pdf file containing tasks)
pub struct Sheet {
    pub tasks_path: PathBuf,
    pub solutions_path: Option<PathBuf>,
    pub tasks: HashMap<String, Task>,
}