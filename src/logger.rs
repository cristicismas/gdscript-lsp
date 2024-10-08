use std::{
    fs::{self, read_to_string, File},
    io::Write,
    path::PathBuf,
};

use home::home_dir;

const LOGS_FOLDER: &str = ".local/state/gdscript-lsp/logs";
const DEFAULT_LOGS_FILE: &str = "logs.txt";

pub fn print_logs(message: String) {
    try_print_logs(message, None).unwrap();
}

pub fn print_logs_to(message: String, file_name: &str) {
    try_print_logs(message, Some(file_name)).unwrap();
}

pub fn print_error(message: String) -> ! {
    try_print_logs(message.clone(), None).unwrap();
    panic!("{}", message);
}

fn try_print_logs(message: String, file_name: Option<&str>) -> std::io::Result<()> {
    let file_location = get_logs_path(file_name);
    fs::create_dir_all(get_logs_folder())?;

    let file_contents = read_to_string(file_location.clone()).unwrap();

    let mut file = File::create(file_location)?;

    let full_message = format!("{}\n{}", file_contents, message);

    file.write_all(&full_message.into_bytes())?;

    Ok(())
}

pub fn clear_logs(file_name: Option<&str>) {
    let file_location = get_logs_path(file_name);

    fs::create_dir_all(get_logs_folder()).unwrap();

    let mut file = File::create(file_location).unwrap();
    file.write_all(&String::from("").into_bytes()).unwrap();
}

fn get_home_dir() -> PathBuf {
    match home_dir() {
        Some(value) => value,
        None => panic!("Unable to get home directory."),
    }
}

fn get_logs_folder() -> PathBuf {
    let home_dir = get_home_dir();

    let logs_folder = home_dir.join(LOGS_FOLDER);

    return logs_folder;
}

fn get_logs_path(file_name: Option<&str>) -> PathBuf {
    let home_dir = get_home_dir();

    let file_name = file_name.unwrap_or(DEFAULT_LOGS_FILE);

    let file_location = home_dir.join(LOGS_FOLDER).join(file_name);

    return file_location;
}
