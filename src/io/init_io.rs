use std::{collections::HashMap, path::PathBuf};

use crate::io::read_modules::read_modules;
use crate::structs::module::Module;

/// Search for the .studystreak.yaml file. If it is found, return the content.
/// If not, it is created and the return value will be empty.
pub fn init_io(io_path: &PathBuf) -> HashMap<String, Module> {
    if io_path.exists() {
        read_modules(io_path)
    } else {
        // todo: create file
        HashMap::new()
    }
}
