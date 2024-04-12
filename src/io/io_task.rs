use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IoTask {
    pub topic: Option<String>,
    pub attempts: Vec<String>,
    pub subtasks: HashMap<String, IoTask>,
    pub position: u32,
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl IoTask {
        pub fn test_default1() -> IoTask {
            IoTask {
                topic: Some("Vectors".to_owned()),
                attempts: vec!["h".to_owned(), "v".to_owned()],
                subtasks: HashMap::from([("a)".to_owned(), IoTask::test_default2())]),
                position: 0,
            }
        }
        pub fn test_default2() -> IoTask {
            IoTask {
                topic: Some("Tractors".to_owned()),
                attempts: vec!["9/11".to_owned()],
                subtasks: HashMap::new(),
                position: 0,
            }
        }
    }
}
