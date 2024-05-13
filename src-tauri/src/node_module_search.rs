
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};
use std::path::Path;

/* use sysinfo::Disks;

#[warn(dead_code)]
fn list_disks() {
    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{disk:?}");
    }
}
*/

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

fn find_node_modules(paths_to_skip: &[&str]) -> Vec<String> {
    let mut file_paths = Vec::new();

    for entry in WalkDir::new("C:\\")
        .into_iter()
        .filter_entry(|e| !paths_to_skip.iter().any(|&path| e.path().to_string_lossy().contains(path)))
        .filter_map(Result::ok)
        .filter(is_node_modules) {
        println!("{}", entry.path().display());
        file_paths.push(entry.path().display().to_string());
    }

    return file_paths;
}

#[tauri::command]
pub fn list_node_modules() -> Vec<String> {
    let start_time = Instant::now();
    println!("=> node_modules:");
    let paths_to_skip = vec![".vscode", "AppData", "nodejs", "Program Files"];
    let paths = find_node_modules(&paths_to_skip);
    let end_time = Instant::now();
    println!("Elapsed time: {:?}", end_time - start_time);

    return paths;
}