use std::{collections::HashMap, path::PathBuf};
use serde::Deserialize;

use super::io_task::IoTask;

#[derive(Debug, Deserialize, PartialEq)]
pub struct IoSheet {
    pub tasks_path: PathBuf,
    pub solutions_path: Option<PathBuf>,
    pub tasks: HashMap<String, IoTask>,
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl IoSheet {
        pub fn test_default1() -> IoSheet {
            IoSheet {
                tasks_path: PathBuf::from("/path/to/tasks.pdf"),
                solutions_path: Some(PathBuf::from("/path/to/solutions.pdf")),
                tasks: HashMap::from([("1.".to_owned(), IoTask::test_default1())]),
            }
        }
    }
}