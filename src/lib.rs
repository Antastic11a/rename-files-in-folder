//! This crate contains a method to replace regex matches of the names of files 
//! and folders within a directory.

use regex::Regex;
use std::fs::{read_dir, rename};
use std::path::PathBuf;

fn regex_match_and_replace(re: &str, input: &str, replacement: &str) -> String {
    Regex::new(re)
        .expect("Regex expresion didn't parse.")
        .replace_all(input, replacement)
        .to_string()
}

/// Replace regex matches of the names of files and folders within a directory.
/// 
/// # Example
///
/// ```
/// use rename_files_in_folder::rename_files_in_folder;
/// use std::path::PathBuf;
/// 
/// let dir_path = PathBuf::from("example/path");
/// rename_files_in_folder(
///     "foo",
///     "bar",
///     &dir_path
/// );
/// ```
/// 
/// After this runs, any files with "foo" in the name within the directory example/path will then
/// have "foo" replaced with "bar". For further example, if example/path has the contents of...
/// 
/// file.foo <br>
/// foofoo <br>
/// foo folder
/// 
/// if the code from above is then ran, the files/folders will be renamed to...
/// 
/// file.bar <br>
/// barbar <br>
/// bar folder
///  
pub fn rename_files_in_folder(
    re: &str,
    replacement: &str,
    dir_path: &PathBuf,
) {
    for dir in read_dir(dir_path)
        .expect("Failed to read_dir on TEST_dir_path.") {
        // Create PathBuf for DirEntry.
        let file_path = dir
            .expect("Failed to get path from DirEntry.")
            .path();
        // Generate new file name.
        let new_file_name = regex_match_and_replace(
            re,
            file_path
                .file_name()
                .expect("Failed to get new_file_name.")
                .to_str()
                .expect("Failed to covert new_file_name to str."),
            replacement
        );
        println!("{:?} renaming to {:?}", file_path.file_name().unwrap(), new_file_name);
        // Replace file path of dir (DirEntry).
        let mut new_file_path = file_path.clone();
        new_file_path.set_file_name(new_file_name);
        // Rename the file
        rename(file_path, new_file_path)
            .expect("Failed to rename file");
    }
    // End
    println!("Done renaming!");
}