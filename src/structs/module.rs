use std::collections::{HashMap, HashSet};

use crate::io::io_module::IoModule;
use super::sheet::Sheet;
use super::attempt::UnsupportedAttemptStringError;
slint::include_modules!();

#[derive(Debug, PartialEq)]
/// A university module like Basic Mathematics 1 or Electrical Engineering
pub struct Module {
    pub sheets: HashMap<String, Sheet>,
    pub topics: HashSet<String>,
}
impl Module {
    pub fn parse(io_module: &IoModule) -> Result<Module, UnsupportedAttemptStringError> {
        let mut sheets = HashMap::new();
        for (k, v) in &io_module.sheets {
            sheets.insert(k.to_owned(), Sheet::parse(v)?);
        }
    
        let mut topics = HashSet::new();
        for s in sheets.values() {
            topics.extend(s.compile_topics());
        }
    
        Ok(Module {
            sheets,
            topics: topics.iter().map(|t| t.to_string()).collect(),
        })
    }
    
    pub fn progress(&self) -> ProgressValues {
        let mut progress = ProgressValues {
            correct: 0,
            incorrect: 0,
            with_help: 0,  
        };

        self.sheets.iter()
            .for_each(|(_, sheet)| {
                let sheet_progress = sheet.progress();

                progress.correct += sheet_progress.correct;
                progress.with_help += sheet_progress.with_help;
                progress.incorrect += sheet_progress.incorrect;
            });

        progress
    }
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl Module {
        pub fn test_default1() ->  Module {
            Module {
                sheets: HashMap::from([("e01".to_owned(), Sheet::test_default1())]),
                topics: HashSet::from(["Vectors".to_owned(), "Tractors".to_owned()])
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::structs::{attempt::Attempt, sheet::tests::build_sheets_map, task::{tests::build_tasks_map, Task}};

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Module::parse(&IoModule::test_default1()).unwrap(),
            Module::test_default1()
        )
    }

    #[test]
    fn test_progress() {
        let module = Module {
            sheets: build_sheets_map(vec![
                Sheet {
                    tasks: build_tasks_map(vec![
                        Task {
                            attempts: vec![
                                Attempt::Correct
                            ],
                            ..Task::test_default_empty()
                        }
                    ]),
                    ..Sheet::test_default1()
                },
                Sheet {
                    tasks: build_tasks_map(vec![
                        Task {
                            attempts: vec![
                                Attempt::Incorrect
                            ],
                            ..Task::test_default_empty()
                        }
                    ]),
                    ..Sheet::test_default1()
                },
            ]),
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