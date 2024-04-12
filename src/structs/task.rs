use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use slint::{ModelRc, SharedString, VecModel};

use super::attempt::{Attempt, UnsupportedAttemptStringError};
use super::super::io::io_task::IoTask;
use crate::{ProgressValues, SlintTask};

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub topic: Option<String>,
    pub attempts: Vec<Attempt>,
    pub subtasks: HashMap<String, Task>,
    pub position: u32,
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
            subtasks,
            position: io_task.position,
        })
    }

    pub fn to_slint(&self, name: String, depth: u8) -> Vec<SlintTask> {
        let mut slint_tasks = Vec::new();

        slint_tasks.push(
            SlintTask {
                name: SharedString::from(&name.to_string()),
                topic: match &self.topic {
                    Some(t) => SharedString::from(t),
                    None => SharedString::from("")
                },
                attempts: Task::attempts_to_slint(&self.attempts),
                depth: depth as i32,
                position: self.position as i32,
            }
        );

        slint_tasks.extend(Task::tasks_to_slint_vec(&self.subtasks, depth + 1));
        
        slint_tasks
    }

    fn attempts_to_slint(attempts: &Vec<Attempt>) -> ModelRc<SharedString> {
        let mut attempts_rc: Rc<VecModel<SharedString>> = Rc::new(VecModel::default());

        for attempt in attempts {
            attempts_rc.push(SharedString::from(attempt.to_string()));
        }

        ModelRc::from(attempts_rc)
    }

    pub fn tasks_to_slint_vec(tasks: &HashMap<String, Task>, depth: u8) -> Vec<SlintTask> {
        let mut slint_tasks = Vec::new();

        for (task_name, task) in tasks {
            slint_tasks.extend(task.to_slint(task_name.to_string(), depth + 1));
        }

        slint_tasks.sort_by(|task_a, task_b| task_a.position.cmp(&task_b.position));

        slint_tasks
    }

    pub fn topics(&self) -> HashSet<String> { // todo: rename to topics
        let mut topics = HashSet::new();
        if self.topic.is_some() {
            topics.insert(self.topic.as_ref().unwrap().clone());
        }

        self.subtasks
            .iter()
            .for_each(|(_, t)| topics.extend(t.topics()));

        topics
    }

    pub fn progress(&self) -> ProgressValues {
        let mut progress = ProgressValues {
            correct: 0,
            incorrect: 0,
            with_help: 0,  
        };

        if self.subtasks.is_empty() {
            self.attempts.iter()
                .for_each(|attempt| {
                    match attempt {
                        Attempt::Correct => progress.correct += 1,
                        Attempt::Incorrect => progress.incorrect += 1,
                        Attempt::WithHelp => progress.with_help += 1,
                        Attempt::Skipped => {},
                        Attempt::PartiallyCorrect(correct, incorrect) => {
                            progress.correct += *correct as i32;
                            progress.incorrect += *incorrect as i32;
                        }
                    }
                });
        } else {
            self.subtasks
                .values().into_iter()
                .for_each(|subtask| {
                    let subtask_progress = subtask.progress();

                    progress.correct += subtask_progress.correct;
                    progress.with_help += subtask_progress.with_help;
                    progress.incorrect += subtask_progress.incorrect;
                });
        }

        progress
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
                subtasks: HashMap::from([("a)".to_owned(), Task::test_default2())]),
                position: 0,
            }
        }
        pub fn test_default2() -> Task {
            Task {
                topic: Some("Tractors".to_owned()),
                attempts: vec![Attempt::PartiallyCorrect(9, 11)],
                subtasks: HashMap::new(),
                position: 0,
            }
        }
        pub fn test_default_empty() -> Task {
            Task {
                topic: None,
                attempts: Vec::new(),
                subtasks: HashMap::new(),
                position: 0,
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::rc::Rc;

    use slint::{ModelRc, SharedString, VecModel};

    use super::*;

    pub fn build_tasks_map(tasks: Vec<Task>) -> HashMap<String, Task> {
        tasks.iter()
            .enumerate()
            .map(|(index, task)| {
                (index.to_string(), task.clone())
            })
            .collect()
    }

    pub fn build_attempts_rc(attempts: Vec<&str>) -> ModelRc<SharedString> {
        let mut attempts_rc: Rc<VecModel<SharedString>> = Rc::new(VecModel::default());

        for attempt in attempts {
            attempts_rc.push(SharedString::from(attempt));
        }

        ModelRc::from(attempts_rc)
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Task::parse(&IoTask::test_default1()).unwrap(),
            Task::test_default1()
        )
    }

    #[test]
    fn test_topics() {
        assert_eq!(
            Task::test_default2().topics(),
            HashSet::from(["Tractors".to_owned()])
        );
        assert_eq!(
            Task::test_default1().topics(),
            HashSet::from(["Vectors".to_owned(), "Tractors".to_owned()])
        );
    }

    #[test]
    fn test_progress_single_task() {
        assert_eq!(
            Task {
                attempts: vec![
                    Attempt::Correct,
                    Attempt::Correct,
                    Attempt::WithHelp,
                    Attempt::Incorrect,
                ],
                ..Task::test_default_empty()
            }.progress(),
            ProgressValues {
                correct: 2,
                with_help: 1,
                incorrect: 1
            }
        )
    }

    #[test]
    fn test_progress_single_task_skipped() {
        assert_eq!(
            Task {
                attempts: vec![
                    Attempt::Skipped
                ],
                ..Task::test_default_empty()
            }.progress(),
            ProgressValues {
                correct: 0,
                with_help: 0,
                incorrect: 0
            }
        )
    }

    #[test]
    fn test_progress_single_task_partially_correct() {
        assert_eq!(
            Task {
                attempts: vec![
                    Attempt::PartiallyCorrect(5, 3)
                ],
                ..Task::test_default_empty()
            }.progress(),
            ProgressValues {
                correct: 5,
                with_help: 0,
                incorrect: 3
            }
        )
    }

    #[test]
    fn test_progress_subtasks() {
        let task = Task {
            subtasks: build_tasks_map(vec![
                Task {
                    attempts: vec![
                        Attempt::Correct,
                    ],
                    ..Task::test_default_empty()
                },
                Task {
                    attempts: vec![
                        Attempt::WithHelp,
                    ],
                    ..Task::test_default_empty()
                },
            ]),
            ..Task::test_default_empty()
        };

        assert_eq!(
            task.progress(),
            ProgressValues {
                correct: 1,
                with_help: 1,
                incorrect: 0
            }
        )
    }

    #[test]
    fn test_progress_subtasks_and_attempts() {
        let task = Task {
            subtasks: build_tasks_map(vec![
                Task {
                    attempts: vec![
                        Attempt::Correct,
                    ],
                    ..Task::test_default_empty()
                },
            ]),
            attempts: vec![
                Attempt::Incorrect
            ],
            ..Task::test_default_empty()
        };

        assert_eq!(
            task.progress(),
            ProgressValues {
                correct: 1,
                with_help: 0,
                incorrect: 0
            }
        )
    }

}