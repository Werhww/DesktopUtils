extern crate fs_extra;
use fs_extra::dir::get_size;
use std::fs::{remove_dir_all, read_to_string};
use sysinfo::Disks;

mod file_explore;
pub use file_explore::get_dir_files;

#[tauri::command]
pub fn folder_size(path: &str) -> u64 {
    let folder_size = get_size(path).unwrap_or_else( |_| {
        0
    });
    
    println!("Folder size: {}", folder_size);
    return folder_size;   
}

#[tauri::command]
pub fn file_size(path: &str) -> u64 {
    let file_size = get_size(path).unwrap_or_else( |_| {
        0
    });
    
    println!("Folder size: {}", file_size);
    return file_size;   
}

#[tauri::command]
pub fn delete_folder(path: &str) -> bool {
    let result = remove_dir_all(path);
    match result {
        Ok(_) => {
            println!("Folder deleted: {}", path);
            return true;
        },
        Err(e) => {
            println!("Error deleting folder: {}", e);
            return false;
        }
    }
}

pub fn list_disks() -> Vec<String> {
    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    let mut disk_name_list  = Vec::new();

    for disk in disks.list() {
        disk_name_list.push(disk.mount_point().to_string_lossy().to_string());
    }

    return disk_name_list;
}

#[tauri::command]
pub fn read_file(file_path: &str) -> String {
    println!("Reading file: {}", file_path);
    return read_to_string(file_path).unwrap_or_else( |_| {
        "".to_string()
    });
}