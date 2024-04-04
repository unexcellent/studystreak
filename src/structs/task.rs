use std::collections::{HashMap, HashSet};

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
    pub fn compile_topics(&self) -> HashSet<&String> {
        let mut topics = HashSet::new();
        if self.topic.is_some() {
            topics.insert(self.topic.as_ref().unwrap());
        }

        self.subtasks
            .iter()
            .for_each(|(_, t)| topics.extend(t.compile_topics()));

        topics
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

    #[test]
    fn test_compile_topics() {
        assert_eq!(
            Task::test_default2().compile_topics(),
            HashSet::from([&"Tractors".to_owned()])
        );
        assert_eq!(
            Task::test_default1().compile_topics(),
            HashSet::from([&"Vectors".to_owned(), &"Tractors".to_owned()])
        );
    }
}