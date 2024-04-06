use std::{collections::HashMap, path::PathBuf};

use crate::{io::io_root::IoRoot, structs::module::Module};

pub fn read_modules(path: &PathBuf) -> HashMap<String, Module> {
    let raw_yaml_content = IoRoot::from(&path);

    let mut modules = HashMap::new();
    raw_yaml_content.modules.iter()
        .for_each(|(name, io_module)| { modules.insert(name.to_owned(), Module::parse(io_module).unwrap()); } );

    modules
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_read_modules() {
        let path = PathBuf::from("test/assets/test_modules.json");
        assert!(path.exists());

        assert_eq!(
            read_modules(&path).len(),
            2
        )
    }
}