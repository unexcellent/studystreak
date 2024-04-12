use serde::Deserialize;

use super::io_sheet::IoSheet;

#[derive(Debug, Deserialize, PartialEq)]
pub struct IoModule {
    pub name: String,
    pub sheets: Vec<IoSheet>,
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
