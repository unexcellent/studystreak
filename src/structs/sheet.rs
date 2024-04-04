use std::{collections::HashMap, path::PathBuf};

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
    pub fn parse(io_sheet: IoSheet) -> Result<Sheet, UnsupportedAttemptStringError> {
        let mut tasks = HashMap::new();
        for (k, v) in &io_sheet.tasks {
            tasks.insert(k.to_owned(), Task::parse(v)?);
        }
        Ok( Sheet {
            tasks_path: io_sheet.tasks_path,
            solutions_path: io_sheet.solutions_path,
            tasks,
        } )
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
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Sheet::parse(IoSheet::test_default1()).unwrap(),
            Sheet::test_default1()
        )
    }
}