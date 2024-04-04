use std::collections::HashMap;

use super::sheet::Sheet;

/// A university module like Basic Mathematics 1 or Electrical Engineering
pub struct Module {
    sheets: HashMap<String, Sheet>,
    topics: Vec<String>,
}