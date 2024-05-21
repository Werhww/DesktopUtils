extern crate fs_extra;
use fs_extra::dir::get_size;

#[tauri::command]
pub fn folder_size(path: &str) -> u64 {
    let folder_size = get_size(path).unwrap_or_else( |_| {
        0
    });
    
    println!("Folder size: {}", folder_size);
    return folder_size;   
}

#[tauri::command]
pub fn delete_folder(path: &str) -> bool {
    let result = fs_extra::dir::remove(path);
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