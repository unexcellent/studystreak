#![allow(dead_code, unused_variables, unused_mut)] // TODO: remove
slint::include_modules!();

#[path ="structs/mod.rs"]
mod structs;

#[path ="io/mod.rs"]
mod io;

use std::path::PathBuf;

const IO_PATH: &str = "~/.studystreak.yaml";

fn main() -> Result<(), slint::PlatformError> {
    let io_path = PathBuf::from(IO_PATH);
    let mut modules = io::init_io::init_io(&io_path);

    let ui = AppWindow::new()?;
    ui.run()
}