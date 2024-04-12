use std::collections::HashSet;

use super::sheet::Sheet;
use crate::io::io_module::IoModule;
use crate::ProgressValues;

#[derive(Debug, PartialEq, Clone)]
/// A university module like Basic Mathematics 1 or Electrical Engineering
pub struct Module {
    pub name: String,
    pub sheets: Vec<Sheet>,
    pub topics: HashSet<String>,
}

impl From<&IoModule> for Module {
    fn from(io_module: &IoModule) -> Self {
        let sheets: Vec<Sheet> = io_module
            .sheets.iter()
            .map(|io_sheet| Sheet::from(io_sheet))
            .collect();

        let mut topics = HashSet::new();
        sheets.iter()
            .for_each(|sheet| {
                topics.extend(sheet.topics())
            });

        Module {
            name: io_module.name.to_string(),
            sheets,
            topics,
        }
    }
}

impl Module {
    pub fn progress(&self) -> ProgressValues {
        let mut progress = ProgressValues {
            correct: 0,
            incorrect: 0,
            with_help: 0,
        };

        for sheet in &self.sheets {
            let sheet_progress = sheet.progress();

            progress.correct += sheet_progress.correct;
            progress.with_help += sheet_progress.with_help;
            progress.incorrect += sheet_progress.incorrect;
        }

        progress
    }
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl Module {
        pub fn test_default1() -> Module {
            Module {
                name: "Basic Maths 1".to_string(),
                sheets: vec![Sheet::test_default1()],
                topics: HashSet::from(["Vectors".to_owned(), "Tractors".to_owned()]),
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::structs::{
        attempt::Attempt,
        task::{tests::build_tasks_map, Task},
    };

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Module::from(&IoModule::test_default1()),
            Module::test_default1()
        )
    }

    #[test]
    fn test_progress() {
        let module = Module {
            sheets: vec![
                Sheet {
                    tasks: build_tasks_map(vec![Task {
                        attempts: vec![Attempt::Correct],
                        ..Task::test_default_empty()
                    }]),
                    ..Sheet::test_default1()
                },
                Sheet {
                    tasks: build_tasks_map(vec![Task {
                        attempts: vec![Attempt::Incorrect],
                        ..Task::test_default_empty()
                    }]),
                    ..Sheet::test_default1()
                },
            ],
            ..Module::test_default1()
        };

        assert_eq!(
            module.progress(),
            ProgressValues {
                correct: 1,
                with_help: 0,
                incorrect: 1,
            }
        )
    }
}
