#![allow(dead_code, unused_variables, unused_mut, unused_assignments)] // TODO: remove

use std::collections::HashMap;
use std::{path::PathBuf, rc::Rc};
slint::include_modules!();

#[path = "structs/mod.rs"]
mod structs;
use slint::{ModelRc, SharedString, VecModel};
use structs::module::{self, Module};

#[path = "io/mod.rs"]
mod io;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let io_path = PathBuf::from(format!("{}/.studystreak.json5", std::env!("HOME")));
    let mut modules = io::init_io::init_io(&io_path);

    ui.global::<Callbacks>().on_populate_module_page({
        let ui_weak = ui.as_weak().unwrap();
        let module_data = modules.clone();

        move |module_index| {
            let module_data = module_data.clone();

            let module = module_data.get(module_index as usize).unwrap();
            let mut slint_sheets: Rc<VecModel<SlintSheet>> = Rc::new(VecModel::default());
            for (name, sheet) in &module.sheets {
                slint_sheets.push(SlintSheet {
                    name: SharedString::from(name),
                    progress: sheet.progress(),
                });
            }

            ui_weak
                .global::<State>()
                .set_sheets(ModelRc::from(slint_sheets));
        }
    });

    ui.global::<Callbacks>().on_populate_sheet_page({
        let ui_weak = ui.as_weak().unwrap();
        let module_data = modules.clone();

        move |module_index, sheet_id| {
            let module_data = module_data.clone();

            let mut slint_tasks: Rc<VecModel<SlintTask>> = Rc::new(VecModel::default());
            let sheet = module_data
                .get(module_index as usize).unwrap()
                .sheets
                .get(&sheet_id.to_string()).unwrap();

            for (task_name, task) in &sheet.tasks {
                slint_tasks.extend(task.to_slint(task_name.to_string(), 0));
            }

            ui_weak
                .global::<State>()
                .set_tasks(ModelRc::from(slint_tasks));
        }
    });

    ui.global::<Callbacks>()
        .on_indent_based_on_depth(|string, depth| {
            let mut indented_string = String::new();

            for i in 0..depth {
                indented_string.push(' ');
            }

            indented_string.push_str(string.as_ref());

            SharedString::from(indented_string)
        });

    populate_start_page(&modules, &ui);

    ui.run()
}

fn populate_start_page(modules: &Vec<Module>, ui: &AppWindow) {
    let mut slint_modules: Rc<VecModel<SlintModule>> = Rc::new(VecModel::default());
    for module in modules {
        slint_modules.push(SlintModule {
            name: SharedString::from(module.name.to_string()),
            progress: module.progress(),
        });
    }

    ui.global::<State>()
        .set_modules(ModelRc::from(slint_modules));
}
