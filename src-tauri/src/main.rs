// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_positioner::{WindowExt, Position};

mod node_module_search;
use node_module_search::find_node_modules;

mod utils;
use utils::{path_size, delete_folder, read_file,get_dir_files};

mod javascript_manger;
use javascript_manger::{find_package_jsons_entier_computer, find_package_jsons_in_folder};

use tauri::Manager;
use window_shadows::set_shadow;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).unwrap();

            let _ = window.move_window(Position::TopRight);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            find_node_modules,
            path_size,
            delete_folder,
            find_package_jsons_entier_computer,
            find_package_jsons_in_folder,
            read_file,
            get_dir_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
