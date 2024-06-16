use crate::utils::list_disks;
use std::path::Path;
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};

fn is_package_json(entry: &DirEntry) -> bool {
    let file_name = entry.file_name().to_string_lossy();
    if file_name == "package.json" {
        let parent_path = entry.path().parent().unwrap_or(Path::new(""));
        let parent_path_str = parent_path.to_string_lossy().to_string();
        if parent_path_str.contains("node_modules") {
            return false;
        }

        return true;
    }

    false
}

#[tauri::command]
pub fn find_package_jsons_entier_computer(paths_to_skip: Vec<String>) -> Vec<String> {
    let start_time = Instant::now();
    println!("=> node_modules:");

    let mut file_paths = Vec::new();

    let directorys = list_disks();

    for directory in directorys {
        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_entry(|e| -> bool {
                !paths_to_skip.contains(
                    &e.path()
                        .file_name()
                        .unwrap_or(std::ffi::OsStr::new(""))
                        .to_string_lossy()
                        .to_string(),
                )
            })
            .filter_map(Result::ok)
            .filter(is_package_json)
        {
            println!("{}", entry.path().display());
            file_paths.push(entry.path().display().to_string());
        }
    }

    let end_time = Instant::now();
    println!("Elapsed time: {:?}", end_time - start_time);

    return file_paths;
}

#[tauri::command]
pub fn find_package_jsons_in_folder(paths_to_skip: Vec<String>, folder: String) -> Vec<String> {
    let mut file_paths = Vec::new();

    for entry in WalkDir::new(folder)
        .into_iter()
        .filter_entry(|e| -> bool {
            !paths_to_skip.contains(
                &e.path()
                    .file_name()
                    .unwrap_or(std::ffi::OsStr::new(""))
                    .to_string_lossy()
                    .to_string(),
            )
        })
        .filter_map(Result::ok)
        .filter(is_package_json)
    {
        println!("{}", entry.path().display());
        file_paths.push(entry.path().display().to_string());
    }

    return file_paths;
}
