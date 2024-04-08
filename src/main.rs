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

    populate_start_page(&modules, &ui);

    ui.run()
}

fn populate_start_page(modules: &HashMap<String, Module>, ui: &AppWindow) {
    let mut slint_modules: Rc<VecModel<SlintModule>> = Rc::new(VecModel::default());
    for (name, module) in modules {
        slint_modules.push(
            SlintModule {
                name: SharedString::from(name),
                progress: ProgressValues {correct: 1, with_help: 2, incorrect: 3}
            }
        );
    }

    assert_eq!(modules.len(), 11);

    ui.global::<State>().set_modules(ModelRc::from(slint_modules));
}