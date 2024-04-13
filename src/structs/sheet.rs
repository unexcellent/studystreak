use std::{
    collections::HashSet, path::PathBuf
};

use crate::io::io_sheet::IoSheet;
use super::task::Task;
use crate::ProgressValues;

#[derive(Debug, PartialEq, Clone)]
/// An exercise sheet (a single pdf file containing tasks)
pub struct Sheet {
    pub name: String,
    pub tasks_path: PathBuf,
    pub solutions_path: Option<PathBuf>,
    pub tasks: Vec<Task>,
}

impl From<&IoSheet> for Sheet {
    fn from(io_sheet: &IoSheet) -> Self {
        Sheet {
            name: io_sheet.name.to_string(),
            tasks_path: PathBuf::from(&io_sheet.tasks_path),
            solutions_path: io_sheet
                .solutions_path.as_ref()
                .map(|path| PathBuf::from(&path)),
            tasks: io_sheet.tasks.iter()
                .map(|io_task| Task::from(io_task))
                .collect(),
        }
    }
}

impl Sheet {
    pub fn topics(&self) -> HashSet<String> {
        let mut topics = HashSet::new();

        self.tasks
            .iter()
            .for_each(|task| topics.extend(task.topics()));

        topics
    }

    pub fn progress(&self) -> ProgressValues {
        let mut progress = ProgressValues {
            correct: 0,
            incorrect: 0,
            with_help: 0,
        };

        self.tasks.iter().for_each(|task| {
            let task_progress = task.progress();

            progress.correct += task_progress.correct;
            progress.with_help += task_progress.with_help;
            progress.incorrect += task_progress.incorrect;
        });

        progress
    }

    /// Get the nth subtask recursively.
    pub fn get_nth_task(&mut self, n: u32) -> Option<&mut Task> {
        let mut current_index: u32 = 0;
        for task in &mut self.tasks {
            match task.get_nth_subtask(n - current_index) {
                (i, Some(t)) => { return Some(t) },
                (i, None) => { current_index += i; }
            }
        }

        None
    }
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl Sheet {
        pub fn test_default1() -> Sheet {
            Sheet {
                name: "e01".to_string(),
                tasks_path: PathBuf::from("/path/to/tasks.pdf"),
                solutions_path: Some(PathBuf::from("/path/to/solutions.pdf")),
                tasks: vec![Task::test_default1()],
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::{collections::HashSet, vec};

    use crate::structs::attempt::Attempt;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Sheet::from(&IoSheet::test_default1()),
            Sheet::test_default1()
        )
    }

    #[test]
    fn test_topics() {
        assert_eq!(
            Sheet::test_default1().topics(),
            HashSet::from(["Vectors".to_owned(), "Tractors".to_owned()])
        );
    }

    #[test]
    fn test_progress() {
        let sheet = Sheet {
            tasks: vec![
                Task {
                    attempts: vec![Attempt::Correct],
                    ..Task::test_default_empty()
                },
                Task {
                    attempts: vec![Attempt::Incorrect],
                    ..Task::test_default_empty()
                },
            ],
            ..Sheet::test_default1()
        };

        assert_eq!(
            sheet.progress(),
            ProgressValues {
                correct: 1,
                with_help: 0,
                incorrect: 1,
            }
        )
    }

    #[test]
    fn test_get_nth_subtask_single_layer() {
        let mut sheet = Sheet {
            tasks: vec![
                Task {
                    name: "1.".to_string(),
                    ..Task::test_default_empty()
                },
                Task {
                    name: "2.".to_string(),
                    ..Task::test_default_empty()
                },
            ],
            ..Sheet::test_default1()
        };

        assert_eq!(
            sheet.get_nth_task(0).unwrap().name,
            "1.".to_string(),
        )
    }

    #[test]
    fn test_get_nth_subtask_two_layers_first() {
        let mut sheet = Sheet {
            tasks: vec![
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
            ..Sheet::test_default1()
        };

        assert_eq!(
            sheet.get_nth_task(1).unwrap().name,
            "a)".to_string(),
        )
    }

    #[test]
    fn test_get_nth_subtask_two_layers_second() {
        let mut sheet = Sheet {
            tasks: vec![
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
            ..Sheet::test_default1()
        };

        assert_eq!(
            sheet.get_nth_task(2).unwrap().name,
            "b)".to_string(),
        )
    }
}
