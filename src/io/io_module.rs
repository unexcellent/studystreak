use serde::{Deserialize, Serialize};

use crate::structs::module::Module;

use super::io_sheet::IoSheet;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IoModule {
    pub name: String,
    pub sheets: Vec<IoSheet>,
}
impl From<&Module> for IoModule {
    fn from(module: &Module) -> Self {
        IoModule {
            name: module.name.to_string(),
            sheets: module
                .sheets.iter()
                .map(|sheet| IoSheet::from(sheet))
                .collect()
        }
    }
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl IoModule {
        pub fn test_default1() -> IoModule {
            IoModule {
                name: "Basic Maths 1".to_string(),
                sheets: vec![IoSheet::test_default1()],
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_from_module() {
        assert_eq!(
            IoModule::from(&Module::test_default1()),
            IoModule::test_default1()
        )
    }
}
