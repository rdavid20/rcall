use chrono::Local;
use dirs::data_local_dir;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn capture_screenshot() -> std::io::Result<PathBuf> {
    let base_dir = data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rcall")
        .join("screenshots");

    fs::create_dir_all(&base_dir)?;

    let file_name = Local::now().format("screenshot_%Y-%m-%dT%H-%M-%S.png");
    let file_path = base_dir.join(file_name.to_string());

    let status = Command::new("grim")
        .arg("-g")
        .arg(slurp_region()?)
        .arg(&file_path)
        .status()?;

    if status.success() {
        Ok(file_path)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "grim failed",
        ))
    }
}

fn slurp_region() -> std::io::Result<String> {
    let output = Command::new("slurp").output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "slurp failed",
        ))
    }
}
