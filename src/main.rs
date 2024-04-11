#![allow(dead_code, unused_variables, unused_mut, unused_assignments)] // TODO: remove

use std::{path::PathBuf, rc::Rc};
use std::collections::HashMap;
slint::include_modules!();

#[path ="structs/mod.rs"]
mod structs;
use slint::{ModelRc, SharedString, VecModel};
use structs::module::Module;

#[path ="io/mod.rs"]
mod io;


fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let io_path = PathBuf::from(format!("{}/.studystreak.json5", std::env!("HOME")));
    let mut modules = io::init_io::init_io(&io_path);

    ui.global::<Events>().on_populate_module_page({
        let ui_weak = ui.as_weak().unwrap();
        let module_data = modules.clone();

        move |module_id| {
            let module_data = module_data.clone();

            let mut slint_sheets: Rc<VecModel<SlintSheet>> = Rc::new(VecModel::default());
            let module = module_data.get(&module_id.to_string()).unwrap();

            for (name, sheet) in &module.sheets {
                slint_sheets.push(SlintSheet {
                    name: SharedString::from(name),
                    progress: sheet.progress()
                });
            }

            ui_weak.global::<State>().set_sheets(ModelRc::from(slint_sheets));
        }
    });

    populate_start_page(&modules, &ui);

    ui.run()
}

fn populate_start_page(modules: &HashMap<String, Module>, ui: &AppWindow) {
    let mut slint_modules: Rc<VecModel<SlintModule>> = Rc::new(VecModel::default());
    for (name, module) in modules {
        slint_modules.push(
            SlintModule {
                name: SharedString::from(name),
                progress: module.progress()
            }
        );
    }

    ui.global::<State>().set_modules(ModelRc::from(slint_modules));
}

// fn populate_module_page(module: &Module, mut ui: AppWindow) {
//     let mut slint_sheets: Rc<VecModel<SlintSheet>> = Rc::new(VecModel::default());

//     for (name, sheet) in &module.sheets {
//         slint_sheets.push(SlintSheet {
//             name: SharedString::from(name),
//             progress: sheet.progress()
//         });
//     }

//     ui.global::<State>().set_sheets(ModelRc::from(slint_sheets));
// }

// fn set_event_methods(modules: HashMap<String, Module>, ui: &AppWindow) {
//     ui.global::<Events>().on_populate_module_page(|module_id| {
//         let mut slint_sheets: Rc<VecModel<SlintSheet>> = Rc::new(VecModel::default());
//         let module = modules.get(&module_id.to_string()).unwrap();

//         for (name, sheet) in &module.sheets {
//             slint_sheets.push(SlintSheet {
//                 name: SharedString::from(name),
//                 progress: sheet.progress()
//             });
//         }

//         ui.global::<State>().set_sheets(ModelRc::from(slint_sheets));
//     });
// }