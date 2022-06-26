use std::fs::{File, remove_file};
use std::path::{Display, Path};
use std::io::{Error, prelude::*};

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
pub fn read_file(filename: &str) -> Result<String, Error> {
    let path: &Path = Path::new(filename);

    let mut file: File = match File::open(&path) {
        Ok(file) => file,
        Err(why) => return Err(why),
    };

    let mut contents: String = String::new();
    return match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(why) => Err(why),
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
        Err(why) => panic!("couldn't delete {}: {}", display, why),
    }
}

#[test]
fn test_file() {
    let filename: String = String::from("test_file.txt");
    let text: String = String::from("This is a test file.");

    write_file(&filename, &text);

    let read: Result<String, Error> = read_file(&filename);

    match read {
        Ok(contents) => {
            assert_eq!(text, contents);
            delete_file(&filename);
        },
        Err(why) => {
            panic!("couldn't read file: {} {}", filename, why);
        },
    }
}
