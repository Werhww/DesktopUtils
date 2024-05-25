
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};
use std::path::Path;

use crate::utils::list_disks;


fn is_node_modules(entry: &DirEntry) -> bool {
    let file_name = entry.file_name().to_string_lossy();
    if file_name == "node_modules" {
        let parent_path = entry.path().parent().unwrap_or(Path::new(""));
        let parent_path_str = parent_path.to_string_lossy().to_string();
        if parent_path_str.contains("node_modules") {
            return false;
        }

        return true;
    }

    false
}

fn find_node_modules(paths_to_skip: Vec<String>) -> Vec<String> {
    let mut file_paths = Vec::new();
 
    let directorys = list_disks();

    for directory in directorys {
        for entry in WalkDir::new(directory)
        .into_iter()
        .filter_entry(|e| -> bool {
            !paths_to_skip.contains(&e.path().file_name().unwrap_or(std::ffi::OsStr::new("")).to_string_lossy().to_string())
        })
        .filter_map(Result::ok)
        .filter(is_node_modules) {
        println!("{}", entry.path().display());
            file_paths.push(entry.path().display().to_string());
        }
    }

    return file_paths;
}

#[tauri::command]
pub fn list_node_modules(paths_to_skip: Vec<String>) -> Vec<String> {
    let start_time = Instant::now();
    println!("=> node_modules:");
    let paths = find_node_modules(paths_to_skip);
    let end_time = Instant::now();
    println!("Elapsed time: {:?}", end_time - start_time);

    return paths;
}