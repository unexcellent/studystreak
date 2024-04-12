use std::path::PathBuf;

use crate::{io::io_root::IoRoot, structs::module::Module};

pub fn read_modules(path: &PathBuf) -> Vec<Module> {
    IoRoot::from(path).modules.iter()
        .map(|io_module| Module::from(io_module))
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_read_modules() {
        let path = PathBuf::from("test/assets/test_modules.json");
        assert!(path.exists());

        assert_eq!(read_modules(&path).len(), 2)
    }
}
