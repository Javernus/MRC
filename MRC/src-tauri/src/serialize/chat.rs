use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    id: u64,
    group_id: u64,
    time: u64,
    name: String,
    message: String,
}

trait CreateFile {
    fn create_file(path: &str);
}

impl Chat {
    pub fn new(id: u64, group_id: u64, time: u64, name: &str, message: &str) -> Chat {
        Chat {
            id,
            group_id,
            time,
            name: String::from(name),
            message: String::from(message),
        }
    }

    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn deserialize(json: &str) -> Chat {
        serde_json::from_str(json).unwrap()
    }

    pub fn create_file(path: &str) -> File {
        File::create(path)?
    }

    pub fn write_to_file(&self, path: &str) {
        let mut file: File = File::open(path)?;
        file.write_all(self.serialize().as_bytes())?;
    }

    pub fn read_from_file(&self, path: &str) -> String {
        let mut file: File = File::open(path)?;
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)?;
        contents
    }
}
