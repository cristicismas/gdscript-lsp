use std::{
    fs::{self, read_to_string, File},
    io::Write,
};

const LOGS_FOLDER: &str = "/home/cristi/personal/godot-lsp/logs";
const DEFAULT_LOGS_FILE: &str = "logs.txt";

pub fn print_logs(message: String, file_name: Option<&str>) -> std::io::Result<()> {
    fs::create_dir_all(LOGS_FOLDER)?;

    let file_name = file_name.unwrap_or(DEFAULT_LOGS_FILE);
    let file_location = format!("{}/{}", LOGS_FOLDER, file_name);

    let file_contents = read_to_string(file_location.clone()).unwrap();

    let mut file = File::create(file_location)?;

    let full_message = format!("{}\n{}", file_contents, message);

    file.write_all(&full_message.into_bytes())?;

    Ok(())
}

pub fn clear_logs(file_name: Option<&str>) -> std::io::Result<()> {
    fs::create_dir_all(LOGS_FOLDER)?;

    let file_name = file_name.unwrap_or(DEFAULT_LOGS_FILE);

    let file_location = format!("{}/{}", LOGS_FOLDER, file_name);

    let mut file = File::create(file_location)?;
    file.write_all(&String::from("").into_bytes())?;

    Ok(())
}
