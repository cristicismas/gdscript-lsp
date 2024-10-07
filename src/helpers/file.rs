use core::panic;
use std::path::{Path, PathBuf};

use crate::logger;

enum ProjectFileError {
    RootFindFailure,
    NoProjectParent,
}

const ROOT_DIRECTORY_LOOKUP_FILE: &str = "project.godot";
const DIRECTORY_TRAVERSAL_LIMIT: u8 = 50;
// Cut this amount of characters from the beginning of file_uri, to create a valid path, since
// file_uri starts with file:// (7 characters)
const CHARACTERS_TO_CUT: usize = 7;

pub fn get_project_directory(file_uri: &str) -> Option<&Path> {
    let valid_uri = &file_uri[CHARACTERS_TO_CUT..];

    let file_path: &Path = Path::new(valid_uri);

    let project_parent = match recursive_find_project_parent(file_path, 0) {
        Ok(v) => Some(v),
        Err(e) => match e {
            ProjectFileError::RootFindFailure => {
                logger::print_logs(
                    "Warning: Maximum recursions reached. Project nesting is too high.".to_string(),
                );

                None
            }
            ProjectFileError::NoProjectParent => None,
        },
    };

    return project_parent;
}

fn is_project_parent(uri: &Path) -> bool {
    let directory: Option<&Path> = if uri.is_dir() {
        Some(uri)
    } else {
        uri.parent()
    };

    if directory.is_none() {
        return false;
    }

    for entry in directory
        .unwrap()
        .read_dir()
        .expect("Failed to read uri directory.")
    {
        if let Ok(entry) = entry {
            let entry_path: PathBuf = entry.path();
            if entry_path.ends_with(ROOT_DIRECTORY_LOOKUP_FILE) {
                return true;
            }
        }
    }

    return false;
}

fn recursive_find_project_parent(
    file_path: &Path,
    current_recursions: u8,
) -> Result<&Path, ProjectFileError> {
    if current_recursions >= DIRECTORY_TRAVERSAL_LIMIT {
        return Err(ProjectFileError::RootFindFailure);
    }

    if is_project_parent(file_path) {
        return Ok(file_path);
    } else {
        let maybe_parent = file_path.parent();

        if let Some(parent) = maybe_parent {
            return recursive_find_project_parent(parent, current_recursions + 1);
        } else {
            return Err(ProjectFileError::NoProjectParent);
        }
    }
}
