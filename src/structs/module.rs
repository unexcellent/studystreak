use std::collections::HashMap;

use super::sheet::Sheet;

/// A university module like Basic Mathematics 1 or Electrical Engineering
pub struct Module {
    pub sheets: HashMap<String, Sheet>,
    pub topics: Vec<String>,
}