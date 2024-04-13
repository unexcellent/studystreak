#![allow(dead_code, unused_variables, unused_mut, unused_assignments)] // TODO: remove

use std::{cell::RefCell, path::PathBuf, rc::Rc};
slint::include_modules!();

#[path = "structs/mod.rs"]
mod structs;
use slint::{ModelRc, SharedString, VecModel};
use structs::module::Module;

#[path = "io/mod.rs"]
mod io;

struct AppState {
    ui: slint::Weak<AppWindow>,
    modules: Rc<Vec<Module>>,
}

fn main() {
    let io_path = PathBuf::from(format!("{}/.studystreak.json5", std::env!("HOME")));
    let modules = io::init_io::init_io(&io_path);

    let ui = AppWindow::new().unwrap();

    let state = Rc::new(RefCell::new(AppState {
        ui: ui.as_weak(),
        modules: Rc::new(modules)
    }));

    let state_copy = state.clone();
    ui.global::<Callbacks>().on_populate_module_page(move |module_index| {
        let binding = state_copy.borrow();
        let module: &Module = binding.modules.get(module_index as usize).unwrap();

        let mut slint_sheets: Rc<VecModel<SlintSheet>> = Rc::new(VecModel::default());
        for sheet in &module.sheets {
            slint_sheets.push(SlintSheet {
                name: SharedString::from(&sheet.name),
                progress: sheet.progress(),
            });
        }

        state_copy.borrow().ui.upgrade().unwrap()
            .global::<State>()
            .set_sheets(ModelRc::from(slint_sheets));
    });

    let state_copy = state.clone();
    ui.global::<Callbacks>().on_populate_sheet_page(move |module_index, sheet_index| {
        let binding = state_copy.borrow();
        let sheet = binding
            .modules
            .get(module_index as usize).unwrap()
            .sheets
            .get(sheet_index as usize).unwrap();

        let mut slint_tasks: Rc<VecModel<SlintTask>> = Rc::new(VecModel::default());
        for task in &sheet.tasks {
            slint_tasks.extend(task.to_slint(0));
        }

        state_copy.borrow().ui.upgrade().unwrap()
            .global::<State>()
            .set_tasks(ModelRc::from(slint_tasks));
    });

    let state_copy = state.clone();
    populate_start_page(state_copy);

    ui.run().unwrap();
}

fn populate_start_page(state: Rc<RefCell<AppState>>) {
    let binding = state.borrow();
    let mut slint_modules: Rc<VecModel<SlintModule>> = Rc::new(VecModel::default());
    for module in binding.modules.iter() {
        slint_modules.push(SlintModule {
            name: SharedString::from(module.name.to_string()),
            progress: module.progress(),
        });
    }

    binding.ui.upgrade().unwrap().global::<State>()
        .set_modules(ModelRc::from(slint_modules));
}
