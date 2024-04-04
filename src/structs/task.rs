use std::collections::HashMap;

use super::attempt::{Attempt, UnsupportedAttemptStringError};
use super::super::io::io_task::IoTask;

#[derive(Debug, PartialEq)]
pub struct Task {
    pub topic: Option<String>,
    pub attempts: Vec<Attempt>,
    pub subtasks: HashMap<String, Task>
}
impl Task {
    pub fn parse(io_task: &IoTask) -> Result<Task, UnsupportedAttemptStringError> {
        let mut attempts = Vec::new();
        for s in &io_task.attempts {
            attempts.push(Attempt::parse(s)?);
        }

        let mut subtasks = HashMap::new();
        for (k, v) in &io_task.subtasks {
            subtasks.insert(k.to_owned(), Task::parse(v)?);
        }

        Ok(Task {
            topic: io_task.topic.clone(),
            attempts,
            subtasks
        })
    }
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl Task {
        pub fn test_default1() -> Task {
            Task {
                topic: Some("Vectors".to_owned()),
                attempts: vec![Attempt::WithHelp, Attempt::Correct],
                subtasks: HashMap::from([("a)".to_owned(), Task::test_default2())])
            }
        }
        pub fn test_default2() -> Task {
            Task {
                topic: Some("Tractors".to_owned()),
                attempts: vec![Attempt::PartiallyCorrect(9, 11)],
                subtasks: HashMap::new()
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Task::parse(&IoTask::test_default1()).unwrap(),
            Task::test_default1()
        )
    }

}