use std::path::PathBuf;
use std::fs::remove_dir_all;
use std::fs::DirBuilder;
use std::fs::File;


pub fn create_folder_and_files(folder_path: &PathBuf, file_names: &[&str]) {
    // Remove old test
    if folder_path.exists() {
        let _ = remove_dir_all(folder_path);
    }
    // Create test_folder
    let _ = DirBuilder::new()
        .create(folder_path);
    // Create files from filenames
    for file_name in file_names {
        // Create file path
        let file_path = folder_path.join(file_name);
        // Create file
        let _ = File::create(file_path);
    }
}