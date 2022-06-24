use std::fs::File;
use std::path::{Display, Path};
use std::io::{prelude::*, Result, BufRead, BufReader, Lines};

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

#[test]
fn test_file() {
    let filename: &str = "test_file123.txt";
    let text: &str = "Hello world 123";
    write_file(filename, text);
    let contents: String = read_file(filename);

    assert_eq!(text, contents);
}