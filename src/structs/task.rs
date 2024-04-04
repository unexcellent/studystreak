use std::collections::HashMap;

use super::attempt::Attempt;

pub struct Task {
    topic: Option<String>,
    attempts: Vec<Attempt>,
    subtasks: HashMap<String, Task>
}