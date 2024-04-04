use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct IoTask {
    pub topic: Option<String>,
    pub attempts: Vec<String>,
    pub subtasks: HashMap<String, IoTask>
}