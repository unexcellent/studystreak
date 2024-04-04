use std::{collections::{HashMap, HashSet}, path::PathBuf};

use crate::io::io_sheet::IoSheet;

use super::task::Task;
use super::attempt::UnsupportedAttemptStringError;


#[derive(Debug, PartialEq)]
/// An exercise sheet (a single pdf file containing tasks)
pub struct Sheet {
    pub tasks_path: PathBuf,
    pub solutions_path: Option<PathBuf>,
    pub tasks: HashMap<String, Task>,
}
impl Sheet {
    pub fn parse(io_sheet: &IoSheet) -> Result<Sheet, UnsupportedAttemptStringError> {
        let mut tasks = HashMap::new();
        for (k, v) in &io_sheet.tasks {
            tasks.insert(k.to_owned(), Task::parse(v)?);
        }
        Ok( Sheet {
            tasks_path: PathBuf::from(&io_sheet.tasks_path),
            solutions_path: match &io_sheet.solutions_path {
                Some(path) => Some(PathBuf::from(&path)),
                None => None
            },
            tasks,
        } )
    }
    pub fn compile_topics(&self) -> HashSet<&String> {
        let mut topics = HashSet::new();

        self.tasks
            .iter()
            .for_each(|(_, t)| topics.extend(t.compile_topics()));

        topics
    }
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl Sheet {
        pub fn test_default1() -> Sheet {
            Sheet {
                tasks_path: PathBuf::from("/path/to/tasks.pdf"),
                solutions_path: Some(PathBuf::from("/path/to/solutions.pdf")),
                tasks: HashMap::from([("1.".to_owned(), Task::test_default1())]),
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Sheet::parse(&IoSheet::test_default1()).unwrap(),
            Sheet::test_default1()
        )
    }

    #[test]
    fn test_compile_topics() {
        assert_eq!(
            Sheet::test_default1().compile_topics(),
            HashSet::from([&"Vectors".to_owned(), &"Tractors".to_owned()])
        );
    }
}