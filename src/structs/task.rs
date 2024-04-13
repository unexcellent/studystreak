use std::collections::HashSet;
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
    pub subtasks: Vec<Task>,
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
            subtasks: io_task
                .subtasks.iter()
                .map(|io_subtask| Task::from(io_subtask))
                .collect(),
        }
    }
}

impl Task {
    pub fn to_slint(&self, depth: u8) -> Vec<SlintTask> {

        let mut slint_tasks = Vec::new();

        slint_tasks.push(SlintTask {
            name: SharedString::from(self.name.to_string()),
            topic: match &self.topic {
                Some(t) => SharedString::from(t),
                None => SharedString::from(""),
            },
            attempts: Task::attempts_to_slint(&self.attempts),
            depth: depth as i32,
        });


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

    fn tasks_to_slint_vec(tasks: &Vec<Task>, depth: u8) -> Vec<SlintTask> {
        let mut slint_tasks = Vec::new();

        for task in tasks {
            slint_tasks.extend(task.to_slint(depth + 1));
        }

        slint_tasks
    }

    pub fn topics(&self) -> HashSet<String> {
        let mut topics = HashSet::new();
        if self.topic.is_some() {
            topics.insert(self.topic.as_ref().unwrap().clone());
        }

        self.subtasks
            .iter()
            .for_each(|subtask| topics.extend(subtask.topics()));

        topics
    }

    pub fn progress(&self) -> ProgressValues {
        let mut progress = ProgressValues {
            correct: 0,
            incorrect: 0,
            with_help: 0,
        };

        if self.subtasks.is_empty() {
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
        } else {
            self.subtasks.iter().for_each(|subtask| {
                let subtask_progress = subtask.progress();

                progress.correct += subtask_progress.correct;
                progress.with_help += subtask_progress.with_help;
                progress.incorrect += subtask_progress.incorrect;
            });
        }

        progress
    }

    /// Return the number of subtasks recursively. The subtasks of subtasks and the subtasks of 
    /// those subtasks (and so on) are also counted.
    pub fn subtask_number(&self) -> u32 {
        self.subtasks.iter()
            .map(|subtask| subtask.subtask_number())
            .sum::<u32>() + 1
    }

    /// Get the nth subtask recursively.
    pub fn get_nth_subtask(&mut self, n: u32) -> (u32, Option<&mut Task>) {
        if n == 0 {
            return (0, Some(self));
        }

        let mut current_index: u32 = 1;
        for subtask in &mut self.subtasks {
            match subtask.get_nth_subtask(n - current_index) {
                (i, Some(t)) => { return (current_index + i, Some(t)) },
                (i, None) => { current_index += i; }
            }
        }

        (current_index, None)
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
                subtasks: vec![Task::test_default2()],
            }
        }
        pub fn test_default2() -> Task {
            Task {
                name: "a)".to_string(),
                topic: Some("Tractors".to_owned()),
                attempts: vec![Attempt::PartiallyCorrect(9, 11)],
                subtasks: Vec::new(),
            }
        }
        pub fn test_default_empty() -> Task {
            Task {
                name: "".to_string(),
                topic: None,
                attempts: Vec::new(),
                subtasks: Vec::new(),
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
            }
            .progress(),
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
                attempts: vec![Attempt::Skipped],
                ..Task::test_default_empty()
            }
            .progress(),
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
                attempts: vec![Attempt::PartiallyCorrect(5, 3)],
                ..Task::test_default_empty()
            }
            .progress(),
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
            subtasks: vec![
                Task {
                    attempts: vec![Attempt::Correct],
                    ..Task::test_default_empty()
                },
                Task {
                    attempts: vec![Attempt::WithHelp],
                    ..Task::test_default_empty()
                },
            ],
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
            subtasks: vec![Task {
                attempts: vec![Attempt::Correct],
                ..Task::test_default_empty()
            }],
            attempts: vec![Attempt::Incorrect],
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

    #[test]
    fn test_subtask_number() {
        assert_eq!(
            Task {
                subtasks: vec![
                    Task {
                        subtasks: vec![],
                        ..Task::test_default_empty()
                    },
                    Task {
                        subtasks: vec![
                            Task {
                                subtasks: vec![],
                                ..Task::test_default_empty()
                            },
                            Task {
                                subtasks: vec![],
                                ..Task::test_default_empty()
                            },
                        ],
                        ..Task::test_default_empty()
                    },
                ],
                ..Task::test_default_empty()
            }.subtask_number(),
            5
        )
    }

    #[test]
    fn test_get_nth_subtask_single_layer() {
        let mut task = Task {
            subtasks: vec![
                Task {
                    name: "1.".to_string(),
                    ..Task::test_default_empty()
                },
                Task {
                    name: "2.".to_string(),
                    ..Task::test_default_empty()
                },
            ],
            ..Task::test_default_empty()
        };

        assert_eq!(
            task.get_nth_subtask(2).1.unwrap().name,
            "2."
        )
    }

    #[test]
    fn test_get_nth_subtask_two_layers_first() {
        let mut task = Task {
            subtasks: vec![
                Task {
                    name: "1.".to_string(),
                    subtasks: vec![
                        Task {
                            name: "a)".to_string(),
                            ..Task::test_default_empty()
                        },
                        Task {
                            name: "b)".to_string(),
                            ..Task::test_default_empty()
                        },
                    ],
                    ..Task::test_default_empty()
                },
            ],
            ..Task::test_default_empty()
        };

        assert_eq!(
            task.get_nth_subtask(2).1.unwrap().name,
            "a)"
        );
    }

    #[test]
    fn test_get_nth_subtask_two_layers_second() {
        let mut task = Task {
            subtasks: vec![
                Task {
                    name: "1.".to_string(),
                    subtasks: vec![
                        Task {
                            name: "a)".to_string(),
                            ..Task::test_default_empty()
                        },
                        Task {
                            name: "b)".to_string(),
                            ..Task::test_default_empty()
                        },
                    ],
                    ..Task::test_default_empty()
                },
            ],
            ..Task::test_default_empty()
        };

        assert_eq!(
            task.get_nth_subtask(3).1.unwrap().name,
            "b)"
        );
    }

}
