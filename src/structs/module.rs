use std::collections::{HashMap, HashSet};

use crate::io::io_module::IoModule;
use super::sheet::Sheet;
use super::attempt::UnsupportedAttemptStringError;

#[derive(Debug, PartialEq)]
/// A university module like Basic Mathematics 1 or Electrical Engineering
pub struct Module {
    pub sheets: HashMap<String, Sheet>,
    pub topics: HashSet<String>,
}
impl Module {
    pub fn parse(io_module: IoModule) -> Result<Module, UnsupportedAttemptStringError> {
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
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Module::parse(IoModule::test_default1()).unwrap(),
            Module::test_default1()
        )
    }
}