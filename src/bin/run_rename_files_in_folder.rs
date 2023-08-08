//! A standalone program to run rename_files_in_folder.

use std::env::{args, current_dir};
use std::path::PathBuf;

/// Run the regex_match_and_replace with the arguments of...
/// 
/// Regular Expression (String) <br>
/// Replacement text (String) (Default: "") <br>
/// Directory (String) (Default: Current Directory)
/// 
fn main() {
    let args: Vec<String> = args().collect();
    let re_arg = args
        .get(1)
        .expect("Didn't find regular expression argument.");
    let replacement_arg = match args.get(2) {
        Some(val) => val,
        None => {
            println!("Didn't find replacement text argument. Will replace with nothing since there's no argument");
            ""
        }
    };
    let folder_path = match args.get(3) {
        Some(val) => PathBuf::from(val),
        None => {
            println!("No path was given, using current dir.");
            current_dir()
                .expect("Couldn't find current_dir.")
        }
    };
    rename_files_in_folder::rename_files_in_folder(&re_arg, &replacement_arg, &folder_path);
    // End
    println!("Finished renaming files!");
}