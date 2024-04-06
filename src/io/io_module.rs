use std::collections::HashMap;
use serde::Deserialize;

use super::io_sheet::IoSheet;

#[derive(Debug, Deserialize, PartialEq)]
pub struct IoModule {
    pub sheets: HashMap<String, IoSheet>,
}

#[cfg(test)]
pub mod test_defaults {
    use super::*;
    impl IoModule {
        pub fn test_default1() -> IoModule {
            IoModule {
                sheets: HashMap::from([("e01".to_owned(), IoSheet::test_default1())]),
            }
        }
    }
}