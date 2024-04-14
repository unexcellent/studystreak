use serde::Deserialize;

use super::io_task::IoTask;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IoSheet {
    pub name: String,
    pub tasks_path: String,
    pub solutions_path: Option<String>,
    pub tasks: Vec<IoTask>,
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    
    impl IoSheet {
        pub fn test_default1() -> IoSheet {
            IoSheet {
                name: "e01".to_string(),
                tasks_path: "/path/to/tasks.pdf".to_owned(),
                solutions_path: Some("/path/to/solutions.pdf".to_owned()),
                tasks: vec![
                    IoTask::test_default1(),
                    IoTask::test_default2(),
                ],
            }
        }
    }
}
