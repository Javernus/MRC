use std::fs::{File, remove_file};
use std::path::{Display, Path};
use std::io::{prelude::*};

/// Creates new file and writes a string to it.
///
/// # Arguments
///
/// * `filename`: filename of new file.
/// * `text`: string to write to file.
///
/// returns: ()
pub fn write_file(filename: &str, text: &str) {
    let path: &Path = Path::new(filename);
    let display: Display = path.display();

    let mut file: File = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", display, why),
    };

    match file.write_all(text.as_bytes()) {
        Ok(_) => println!("successfully wrote to {}", display),
        Err(why) => panic!("couldn't write to {}: {}", display, why),
    }
}

/// Reads from file. Returns the content.
///
/// # Arguments
///
/// * `filename`: filename of file to read.
///
/// returns: String
pub fn read_file(filename: &str) -> String {
    let path: &Path = Path::new(filename);
    let display: Display = path.display();

    let mut file: File = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", display, why),
    };

    let mut contents: String = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => return contents,
        Err(why) => panic!("couldn't read {}: {}", display, why),
    }
}

/// Deletes file.
///
/// # Arguments
///
/// * `filename`: filename of file to delete.
///
/// returns: ()
pub fn delete_file(filename: &str) {
    let path: &Path = Path::new(filename);
    let display: Display = path.display();

    match remove_file(&path) {
        Ok(_) => println!("successfully deleted {}", display),
        Err(why) => panic!("couldn't write to {}: {}", display, why),
    }
}