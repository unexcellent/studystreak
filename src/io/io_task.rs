use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq)]
pub struct IoTask {
    pub topic: Option<String>,
    pub attempts: Vec<String>,
    pub subtasks: HashMap<String, IoTask>
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl IoTask {
        pub fn test_default1() -> IoTask {
            IoTask {
                topic: Some("Vectors".to_owned()),
                attempts: vec!["h".to_owned(), "v".to_owned()],
                subtasks: HashMap::from([("a)".to_owned(), IoTask::test_default2())])
            }
        }
        pub fn test_default2() -> IoTask {
            IoTask {
                topic: Some("Tractors".to_owned()),
                attempts: vec!["9/11".to_owned()],
                subtasks: HashMap::new()
            }
        }
    }
}