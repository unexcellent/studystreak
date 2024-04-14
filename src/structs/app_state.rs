use std::{cell::RefCell, rc::Rc};
use slint::ComponentHandle;

use crate::{AppWindow, Callbacks, State};

use super::{attempt::Attempt, module::Module, task::Task};

pub struct AppState {
    pub weak_ui: slint::Weak<AppWindow>,
    pub modules: Rc<RefCell<Vec<Module>>>,
}
impl AppState {
    pub fn ui(&self) -> AppWindow {
        self.weak_ui.upgrade().unwrap()
    }

    pub fn get_active_module_index(&self) -> usize {
        self.ui().state().get_active_module_index() as usize
    }
    pub fn get_active_sheet_index(&self) -> usize {
        self.ui().state().get_active_sheet_index() as usize
    }
    pub fn get_active_task_index(&self) -> usize {
        self.ui().state().get_active_task_index() as usize
    }

    pub fn add_attempt(&mut self) {
        let mut modules_binding = self.modules.borrow_mut();
        let active_module = modules_binding.get_mut(self.get_active_module_index()).unwrap();
        let active_sheet = active_module.sheets.get_mut(self.get_active_sheet_index()).unwrap();

        let task = active_sheet.tasks.get_mut(self.get_active_task_index()).unwrap();

        task.attempts.push(Attempt::parse("-").unwrap());
    }

    pub fn add_task(&mut self, subtask_depth: u8) {
        let mut modules_binding = self.modules.borrow_mut();
        let active_module = modules_binding.get_mut(self.get_active_module_index()).unwrap();
        let active_sheet = active_module.sheets.get_mut(self.get_active_sheet_index()).unwrap();

        active_sheet.tasks.push(
            Task {
                name: "".to_string(),
                topic: None,
                attempts: Vec::new(),
                subtask_depth,
            } 
        )
    }
}

impl AppWindow {
    pub fn callbacks(&self) -> Callbacks<'_> {
        self.global::<Callbacks>()
    }
    pub fn state(&self) -> State<'_> {
        self.global::<State>()
    }
}