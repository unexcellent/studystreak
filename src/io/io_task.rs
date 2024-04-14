use serde::{Deserialize, Serialize};

use crate::structs::task::Task;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IoTask {
    pub name: String,
    pub topic: Option<String>,
    pub attempts: Vec<String>,
    pub subtask_depth: u8,
}
impl From<&Task> for IoTask {
    fn from(task: &Task) -> Self {
        IoTask {
            name: task.name.to_string(),
            topic: task.topic.to_owned(),
            attempts: task
                .attempts.iter()
                .map(|attempt| attempt.to_string())
                .collect(),
            subtask_depth: task.subtask_depth
        }
    }
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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_from_task() {
        assert_eq!(
            IoTask::from(&Task::test_default1()),
            IoTask::test_default1()
        )
    }
}
