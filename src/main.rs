#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
slint::include_modules!();
use std::{
    error::Error,
    sync::Arc
};

fn main() -> Result<(), Box<dyn Error>> {
    let window = AppWindow::new()?;
    let ui = Arc::new(window);

    println!("Hello world");

    ui.run()?;
    Ok(())
}
