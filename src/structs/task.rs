use std::rc::Rc;

use slint::{ModelRc, SharedString, VecModel};

use super::super::io::io_task::IoTask;
use super::attempt::Attempt;
use crate::{ProgressValues, SlintTask};

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub name: String,
    pub topic: Option<String>,
    pub attempts: Vec<Attempt>,
    pub subtask_depth: u8,
}

impl From<&IoTask> for Task {
    fn from(io_task: &IoTask) -> Self {
        let mut attempts = Vec::new();
        for attempt_str in &io_task.attempts {
            match Attempt::parse(&attempt_str) {
                Ok(attempt) => {attempts.push(attempt);},
                Err(_) => {continue;}
            }
        }

        Task {
            name: io_task.name.to_string(),
            topic: io_task.topic.clone(),
            attempts,
            subtask_depth: io_task.subtask_depth
        }
    }
}

impl Task {
    pub fn to_slint(&self) -> SlintTask {
        SlintTask {
            name: SharedString::from(self.name.to_string()),
            topic: match &self.topic {
                Some(t) => SharedString::from(t),
                None => SharedString::from(""),
            },
            attempts: Task::attempts_to_slint(&self.attempts),
            depth: self.subtask_depth as i32,
        }
    }

    fn attempts_to_slint(attempts: &Vec<Attempt>) -> ModelRc<SharedString> {
        let mut attempts_rc: Rc<VecModel<SharedString>> = Rc::new(VecModel::default());

        for attempt in attempts {
            attempts_rc.push(SharedString::from(attempt.to_string()));
        }

        ModelRc::from(attempts_rc)
    }

    pub fn progress(&self) -> ProgressValues {
        let mut progress = ProgressValues {
            correct: 0,
            incorrect: 0,
            with_help: 0,
        };

        self.attempts.iter().for_each(|attempt| match attempt {
            Attempt::Correct => progress.correct += 1,
            Attempt::Incorrect => progress.incorrect += 1,
            Attempt::WithHelp => progress.with_help += 1,
            Attempt::Skipped => {}
            Attempt::PartiallyCorrect(correct, incorrect) => {
                progress.correct += *correct as i32;
                progress.incorrect += *incorrect as i32;
            }
        });

        progress
    }

}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl Task {
        pub fn test_default1() -> Task {
            Task {
                name: "1.".to_string(),
                topic: Some("Vectors".to_owned()),
                attempts: vec![Attempt::WithHelp, Attempt::Correct],
                subtask_depth: 0,
            }
        }
        pub fn test_default2() -> Task {
            Task {
                name: "a)".to_string(),
                topic: Some("Tractors".to_owned()),
                attempts: vec![Attempt::PartiallyCorrect(9, 11)],
                subtask_depth: 1,
            }
        }
        pub fn test_default_empty() -> Task {
            Task {
                name: "".to_string(),
                topic: None,
                attempts: Vec::new(),
                subtask_depth: 0,
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::rc::Rc;

    use slint::{ModelRc, SharedString, VecModel};

    use super::*;

    pub fn build_attempts_rc(attempts: Vec<&str>) -> ModelRc<SharedString> {
        let mut attempts_rc: Rc<VecModel<SharedString>> = Rc::new(VecModel::default());

        for attempt in attempts {
            attempts_rc.push(SharedString::from(attempt));
        }

        ModelRc::from(attempts_rc)
    }

    #[test]
    fn test_from_iotask() {
        assert_eq!(
            Task::from(&IoTask::test_default1()),
            Task::test_default1()
        )
    }

    #[test]
    fn test_progress() {
        assert_eq!(
            Task {
                attempts: vec![
                    Attempt::Skipped,

                    Attempt::Correct,
                    Attempt::Correct,

                    Attempt::PartiallyCorrect(5, 3),

                    Attempt::Incorrect,

                    Attempt::WithHelp,
                ],
                ..Task::test_default_empty()
            }
            .progress(),
            ProgressValues {
                correct: 7,
                with_help: 1,
                incorrect: 4
            }
        )
    }
}
