use std::{
    process::Command,
    error::Error,
};

pub fn run(command : &str) -> Result<(), Box<dyn Error>> {
    let out = Command::new(command)
        .output()?;

    if out.status.success() {
        println!("Sucess: {}", String::from_utf8_lossy(&out.stdout));
    } else {
        eprintln!("Failed: {}", String::from_utf8_lossy(&out.stderr));
        std::process::exit(out.status.code().unwrap_or(1));
    }

    Ok(())
}
