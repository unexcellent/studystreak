use serde::Deserialize;
use std::collections::HashMap;

use super::io_sheet::IoSheet;

#[derive(Debug, Deserialize, PartialEq)]
pub struct IoModule {
    pub name: String,
    pub sheets: HashMap<String, IoSheet>,
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl IoModule {
        pub fn test_default1() -> IoModule {
            IoModule {
                name: "Basic Maths 1".to_string(),
                sheets: HashMap::from([("e01".to_owned(), IoSheet::test_default1())]),
            }
        }
    }
}
