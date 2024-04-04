use std::{collections::HashMap, path::PathBuf};

use super::task::Task;

/// An exercise sheet (a single pdf file containing tasks)
pub struct Sheet {
    tasks_path: PathBuf,
    solutions_path: Option<PathBuf>,
    tasks: HashMap<String, Task>,
}