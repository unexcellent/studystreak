#![allow(dead_code)] // TODO: remove
slint::include_modules!();

#[path ="structs/mod.rs"]
mod structs;

#[path ="io/mod.rs"]
mod io;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    ui.run()
}
