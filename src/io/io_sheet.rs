use std::collections::HashMap;
use serde::Deserialize;

use super::io_task::IoTask;

#[derive(Debug, Deserialize, PartialEq)]
pub struct IoSheet {
    pub tasks_path: String,
    pub solutions_path: Option<String>,
    pub tasks: HashMap<String, IoTask>,
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl IoSheet {
        pub fn test_default1() -> IoSheet {
            IoSheet {
                tasks_path: "/path/to/tasks.pdf".to_owned(),
                solutions_path: Some("/path/to/solutions.pdf".to_owned()),
                tasks: HashMap::from([("1.".to_owned(), IoTask::test_default1())]),
            }
        }
    }
}