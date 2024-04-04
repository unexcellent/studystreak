use std::collections::HashMap;

use super::attempt::Attempt;

pub struct Task {
    pub topic: Option<String>,
    pub attempts: Vec<Attempt>,
    pub subtasks: HashMap<String, Task>
}