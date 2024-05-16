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