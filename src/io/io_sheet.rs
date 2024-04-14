use serde::{Deserialize, Serialize};

use crate::structs::sheet::Sheet;

use super::io_task::IoTask;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IoSheet {
    pub name: String,
    pub tasks_path: String,
    pub solutions_path: Option<String>,
    pub tasks: Vec<IoTask>,
}
impl From<&Sheet> for IoSheet {
    fn from(sheet: &Sheet) -> Self {
        IoSheet {
            name: sheet.name.to_string(),
            tasks_path: String::from(sheet.tasks_path.to_str().unwrap()),
            solutions_path: match sheet.solutions_path.to_owned() {
                Some(p) => Some(String::from(p.to_str().unwrap())),
                None => None
            },
            tasks: sheet
                .tasks.iter()
                .map(|task| IoTask::from(task))
                .collect()
        }
    }
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    
    impl IoSheet {
        pub fn test_default1() -> IoSheet {
            IoSheet {
                name: "e01".to_string(),
                tasks_path: "/path/to/tasks.pdf".to_owned(),
                solutions_path: Some("/path/to/solutions.pdf".to_owned()),
                tasks: vec![
                    IoTask::test_default1(),
                    IoTask::test_default2(),
                ],
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_from_sheet() {
        assert_eq!(
            IoSheet::from(&Sheet::test_default1()),
            IoSheet::test_default1()
        )
    }
}