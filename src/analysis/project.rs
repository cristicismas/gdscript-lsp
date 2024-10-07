use crate::{helpers::file::get_project_directory, logger, unwrap_or_return};

use std::path::Path;

pub fn analyze_project(file_uri: &str) -> () {
    let project_directory: &Path = unwrap_or_return!(get_project_directory(file_uri));

    logger::print_logs(format!("project directory: {:?}", project_directory));
}
