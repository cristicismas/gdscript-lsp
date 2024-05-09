use std::{
    fs::{self, File},
    io::Write,
};

const LOGS_FOLDER: &str = "./logs";
const DEFAULT_LOGS_FILE: &str = "logs.txt";

// Usage: print_logs(format!("{}", arg1), "logs.txt");
pub fn print_logs(message: String, file_name: Option<&str>) -> std::io::Result<()> {
    fs::create_dir_all(LOGS_FOLDER)?;

    let file_name = file_name.unwrap_or(DEFAULT_LOGS_FILE);

    let file_location = format!("{}/{}", LOGS_FOLDER, file_name);

    let mut file = File::create(file_location)?;
    file.write_all(&message.into_bytes())?;

    Ok(())
}
