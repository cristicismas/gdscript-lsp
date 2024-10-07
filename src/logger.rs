use std::{
    fs::{self, read_to_string, File},
    io::Write,
};

const LOGS_FOLDER: &str = "/home/cristi/personal/gdscript-lsp/logs";
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
    fs::create_dir_all(LOGS_FOLDER)?;

    let file_name = file_name.unwrap_or(DEFAULT_LOGS_FILE);
    let file_location = format!("{}/{}", LOGS_FOLDER, file_name);

    let file_contents = read_to_string(file_location.clone()).unwrap();

    let mut file = File::create(file_location)?;

    let full_message = format!("{}\n{}", file_contents, message);

    file.write_all(&full_message.into_bytes())?;

    Ok(())
}

pub fn clear_logs(file_name: Option<&str>) {
    fs::create_dir_all(LOGS_FOLDER).unwrap();

    let file_name = file_name.unwrap_or(DEFAULT_LOGS_FILE);

    let file_location = format!("{}/{}", LOGS_FOLDER, file_name);

    let mut file = File::create(file_location).unwrap();
    file.write_all(&String::from("").into_bytes()).unwrap();
}
