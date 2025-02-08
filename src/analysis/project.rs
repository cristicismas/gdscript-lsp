use crate::{
    helpers::file::{get_project_directory, recursive_find_gd_files},
    logger, unwrap_or_return,
};

use std::path::Path;

pub fn analyze_project(file_uri: &str) -> () {
    logger::print_logs(format!("project directory searching..."));
    let project_directory: &Path = unwrap_or_return!(get_project_directory(file_uri));

    logger::print_logs(format!("project directory: {:?}", project_directory));

    let project_files =
        unwrap_or_return!(recursive_find_gd_files(project_directory.to_path_buf(), 0));

    logger::print_logs(format!("project files: {:?}", project_files));
}
