use walkdir::WalkDir;

#[tauri::command]
pub fn get_dir_files(path: &str, paths_to_skip: Vec<String>) -> Vec<String> {
    println!("Reading files in: {}", path);

    let mut file_paths = Vec::new();

    for e in WalkDir::new(path)
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
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            file_paths.push(e.path().display().to_string());
        }
    }

    return file_paths;
}
