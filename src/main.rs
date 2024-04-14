#![allow(dead_code, unused_variables, unused_mut, unused_assignments)] // TODO: remove

use std::{cell::RefCell, path::PathBuf, rc::Rc};
slint::include_modules!();

#[path = "structs/mod.rs"]
mod structs;
use slint::{ModelRc, SharedString, VecModel};
use structs::{attempt::Attempt, module::Module, task::Task};

#[path = "io/mod.rs"]
mod io;

struct AppState {
    weak_ui: slint::Weak<AppWindow>,
    modules: Rc<RefCell<Vec<Module>>>,
}
impl AppState {
    pub fn ui(&self) -> AppWindow {
        self.weak_ui.upgrade().unwrap()
    }

    pub fn get_active_module_index(&self) -> usize {
        self.ui().global::<State>().get_active_module_index() as usize
    }
    pub fn get_active_sheet_index(&self) -> usize {
        self.ui().global::<State>().get_active_sheet_index() as usize
    }
    pub fn get_active_task_index(&self) -> usize {
        self.ui().global::<State>().get_active_task_index() as usize
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

fn main() {
    let io_path = PathBuf::from(format!("{}/.studystreak.json5", std::env!("HOME")));
    let modules = io::init_io::init_io(&io_path);

    let ui = AppWindow::new().unwrap();

    let state = Rc::new(RefCell::new(AppState {
        weak_ui: ui.as_weak(),
        modules: Rc::new(RefCell::new(modules)),
    }));

    let state_copy = state.clone();
    ui.global::<Callbacks>().on_populate_module_page(move |module_index| {
        let mut state_binding = state_copy.try_borrow().unwrap();
        let mut modules_binding = state_binding.modules.try_borrow().unwrap();
        
        let module = modules_binding.get(module_index as usize).unwrap();

        let mut slint_sheets: Rc<VecModel<SlintSheet>> = Rc::new(VecModel::default());
        for sheet in &module.sheets {
            slint_sheets.push(SlintSheet {
                name: SharedString::from(&sheet.name),
                progress: sheet.progress(),
            });
        }

        state_binding.ui()
            .global::<State>()
            .set_sheets(ModelRc::from(slint_sheets));
    });

    let state_copy = state.clone();
    ui.global::<Callbacks>().on_populate_sheet_page(move |module_index, sheet_index| {
        let mut state_binding = state_copy.try_borrow().unwrap();
        let mut modules_binding = state_binding.modules.try_borrow().unwrap();

        let sheet = modules_binding
            .get(module_index as usize).unwrap()
            .sheets
            .get(sheet_index as usize).unwrap();

        let mut slint_tasks: Rc<VecModel<SlintTask>> = Rc::new(VecModel::default());
        for task in &sheet.tasks {
            slint_tasks.push(task.to_slint());
        }

        state_binding.ui()
            .global::<State>()
            .set_tasks(ModelRc::from(slint_tasks));
    });

    let state_copy = state.clone();
    ui.global::<Callbacks>().on_add_attempt(move || { state_copy.borrow_mut().add_attempt() });

    let state_copy = state.clone();
    ui.global::<Callbacks>().on_add_task(move |subtask_depth| {
        state_copy.borrow_mut().add_task(subtask_depth as u8)
    });

    let state_copy = state.clone();
    populate_start_page(state_copy);

    ui.run().unwrap();
}

fn populate_start_page(state: Rc<RefCell<AppState>>) {
    let mut state_binding = state.try_borrow().unwrap();
    let mut modules_binding = state_binding.modules.try_borrow().unwrap();

    let mut slint_modules: Rc<VecModel<SlintModule>> = Rc::new(VecModel::default());
    for module in modules_binding.iter() {
        slint_modules.push(SlintModule {
            name: SharedString::from(module.name.to_string()),
            progress: module.progress(),
        });
    }

    state_binding.ui().global::<State>()
        .set_modules(ModelRc::from(slint_modules));
}
