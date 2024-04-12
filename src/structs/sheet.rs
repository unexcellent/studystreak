use std::{
    collections::HashSet,
    path::PathBuf,
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
}
