mod setup;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub const TEST_FOLDER_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("test_folder"));


#[test]
fn test1() {
    setup::create_folder_and_files(&*TEST_FOLDER_PATH, &[
        "test1 apple",
        "test2 pear",
        "test3 banana",
    ]);
    
    rename_files_in_folder::rename_files_in_folder(
        "[a-z+]+",
        "egg",
        &*TEST_FOLDER_PATH);
}