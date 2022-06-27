use std::{
    fs::{File, remove_file, create_dir, remove_dir},
    io::{Result, prelude::*},
    path::Path,
};
use std::str::Split;

/// Returns directory path of file.
///
/// # Arguments
///
/// * `filename`: name of file to look up.
///
/// returns: String
fn get_directory(filename: &str) -> String {
    let split: Split<&str> = filename.split("/");
    let mut vector: Vec<&str> = split.collect();
    vector.pop();
    vector.join("/").to_owned()
}

/// Creates new file and writes a string to it.
///
/// # Arguments
///
/// * `filename`: filename of new file.
/// * `text`: string to write to file.
///
/// returns: Result<()>
pub fn write_file(filename: &str, text: &str) -> Result<()> {
    let file_path: &Path = Path::new(filename);
    let dir_name: String = get_directory(filename);
    let dir_path: &Path = Path::new(&dir_name);

    if !dir_path.is_dir() {
        match create_dir(&dir_path) {
            Ok(_) => {}
            Err(why) => return Err(why),
        };
    }

    let mut file: File = match File::create(file_path) {
        Ok(f) => f,
        Err(why) => return Err(why),
    };

    return match file.write_all(text.as_bytes()) {
        Ok(_) => Ok(()),
        Err(why) => Err(why),
    }
}

/// Reads from file. Returns the content in result format.
///
/// # Arguments
///
/// * `filename`: filename of file to read.
///
/// returns: Result<String>
pub fn read_file(filename: &str) -> Result<String> {
    let file_path: &Path = Path::new(filename);

    return if file_path.is_file() {
        let mut file: File = match File::open(&file_path) {
            Ok(file) => file,
            Err(why) => return Err(why),
        };

        let mut contents: String = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => Ok(contents),
            Err(why) => Err(why),
        }
    } else {
        match write_file(filename, "") {
            Ok(_) => Ok("".to_string()),
            Err(why) => Err(why),
        }
    }
}

/// Deletes file.
///
/// # Arguments
///
/// * `filename`: filename of file to delete.
///
/// returns: Result<()>
pub fn delete_file(filename: &str) -> Result<()> {
    let file_path: &Path = Path::new(filename);

    return if file_path.is_file() {
        match remove_file(file_path) {
            Ok(_) => Ok(()),
            Err(why) => return Err(why),
        }
    } else {
        Ok(())
    }
}

#[test]
fn test_file() {
    let filename: &str = "test_directory/test_file.txt";
    let text: &str = "This is a test file.";

    write_file(filename, text).expect(&*format!("couldn't write {} to file {}", text, filename));

    match read_file(filename) {
        Ok(contents) => {
            assert_eq!(text, contents);
            delete_file(filename).expect(&*format!("couldn't delete file: {}", filename));
            let dir_name: &str = &*get_directory(filename);
            dbg!(filename, dir_name);
            let dir_path: &Path = Path::new(dir_name);
            remove_dir(dir_path).expect(&*format!("couldn't remove directory: {}", dir_name))
        },
        Err(why) => {
            panic!("couldn't read file: {}: {}", filename, why);
        },
    };
}
