use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IoTask {
    pub name: String,
    pub topic: Option<String>,
    pub attempts: Vec<String>,
    pub subtask_depth: u8,
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl IoTask {
        pub fn test_default1() -> IoTask {
            IoTask {
                name: "1.".to_string(),
                topic: Some("Vectors".to_owned()),
                attempts: vec!["h".to_owned(), "v".to_owned()],
                subtask_depth: 0
            }
        }
        pub fn test_default2() -> IoTask {
            IoTask {
                name: "a)".to_string(),
                topic: Some("Tractors".to_owned()),
                attempts: vec!["9/11".to_owned()],
                subtask_depth: 1
            }
        }
    }
}
