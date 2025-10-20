#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
slint::include_modules!();
use ini::Ini;
use std::{
    error::Error,
    sync::Arc,
    path::{
        Path,
        PathBuf
    },
};

mod shell;

fn main() -> Result<(), Box<dyn Error>> {
    let window = AppWindow::new()?;
    let ui = Arc::new(window);

    println!("Hello world");

    let conf = Ini::load_from_file("initest.ini").unwrap();

    for (sec, prop) in &conf {
        println!("Section {:?}", sec);

        for (key, value) in prop.iter() {
            println!("{:?}:{:?}", key, value);
        };
    };

    {
        let ui_copy = ui.clone();
        let ui_weak = ui.as_weak();
        
        let sec = conf.section(Some("ImageTest")).unwrap();

        let path = sec.get("path").unwrap();
        println!("img path: {:?}", path);

        if let Some(ui) = ui_weak.upgrade() {
            let path = PathBuf::from(&path);
            let imageTest = slint::Image::load_from_path(&path).unwrap();
            ui.set_imageTest(imageTest);
        };

        shell::run("ls");

    }


    ui.run()?;
    Ok(())
}
