#![allow(dead_code, unused_variables, unused_mut, unused_assignments)] // TODO: remove

use std::{cell::RefCell, path::PathBuf, rc::Rc};
slint::include_modules!();

#[path = "structs/mod.rs"]
mod structs;
use slint::{ModelRc, SharedString, VecModel};
use structs::app_state::AppState;

#[path = "io/mod.rs"]
mod io;

fn main() {
    let ui = AppWindow::new().unwrap();
    let state = Rc::new(RefCell::new(
        AppState::init(
            PathBuf::from(format!("{}/.studystreak.json5", std::env!("HOME"))),
            ui.as_weak(),
        )
    ));

    let state_copy = state.clone();
    ui.callbacks().on_populate_module_page(move |module_index| {
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
            .state()
            .set_sheets(ModelRc::from(slint_sheets));
    });

    let state_copy = state.clone();
    ui.callbacks().on_populate_sheet_page(move |module_index, sheet_index| {
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
            .state()
            .set_tasks(ModelRc::from(slint_tasks));
    });

    let state_copy = state.clone();
    ui.callbacks().on_add_attempt(move || { state_copy.borrow_mut().add_attempt() });

    let state_copy = state.clone();
    ui.callbacks().on_add_task(move |subtask_depth| {
        state_copy.borrow_mut().add_task(subtask_depth as u8)
    });

    let state_copy = state.clone();
    ui.callbacks().on_edit_task_name(move |task_name| {
        state_copy.borrow_mut().edit_task_name(task_name);
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

    state_binding.ui().state()
        .set_modules(ModelRc::from(slint_modules));
}
