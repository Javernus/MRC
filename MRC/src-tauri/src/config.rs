use crate::file;
use serde::{Serialize, Deserialize};

pub const DEFAULT_USERNAME: &str = "";
pub const DEFAULT_MPW: &str = "";

/// Returns string representation of path to config file.
/// Output: ../data/config.json
///
/// returns: String
fn config_path() -> String {
    String::from("../data/config.json")
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    username: String,
    mpw: String,
}

impl Config {
    fn new(username: &str, mpw: &str) -> Config {
        Config {
            username: username.to_string(),
            mpw: mpw.to_string(),
        }
    }

    fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }

    fn set_mpw(&mut self, mpw: &str) {
        self.mpw = mpw.to_string();
    }

    fn get_username(&self) -> String {
        self.username.to_string()
    }

    fn get_mpw(&self) -> String {
        self.mpw.to_string()
    }
}

fn serialize(config: &Config) -> Result<String, serde_json::Error> {
    serde_json::to_string(config)
}

fn deserialize(text: &str) -> Result<Config, serde_json::Error> {
    if text.is_empty() {
        Ok(Config::new(DEFAULT_USERNAME, DEFAULT_MPW))
    } else {
        serde_json::from_str(text)
    }
}

#[test]
fn test_config_serialization() {
    let config: Config = Config::new("Alice", "password123");
    let ser: String = serialize(&config).unwrap();
    assert_eq!(ser, "{\"username\":\"Alice\",\"mpw\":\"password123\"}");
    let deser: Config = deserialize(&ser).unwrap();
    assert_eq!(config, deser);
}

fn read_config() -> Result<Config, std::io::Error> {
    match file::read_file(&config_path()) {
        Ok(contents) => match deserialize(&contents) {
            Ok(config) => Ok(config),
            Err(why) => Err(std::io::Error::from(why)),
        },
        Err(why) => Err(why),
    }
}

fn write_config(config: &Config) -> Result<(), std::io::Error> {
    match serialize(&config) {
        Ok(serialized) => {
            match file::write_file(&config_path(), &serialized) {
                Ok(_) => Ok(()),
                Err(why) => Err(why),
            }
        }
        Err(why) => Err(std::io::Error::from(why)),
    }
}

/// Retrieves username from config.
/// If no username is found, DEFAULT_USERNAME is set as username and gets returned as well.
///
/// returns: String
pub fn read_username() -> String {
    match read_config() {
        Ok(config) => config.get_username(),
        Err(_) => DEFAULT_USERNAME.to_string(),
    }
}

pub fn read_mpw() -> String {
    match read_config() {
        Ok(config) => config.get_mpw(),
        Err(_) => DEFAULT_MPW.to_string(),
    }
}

/// Sets username and saves it in config.
///
/// # Arguments
///
/// * `username`: username to set in config.
pub fn write_username(username: &str) -> Result<(), std::io::Error> {
    let config: Config = match read_config() {
        Ok(old_config) => {
            let mut new_config = old_config;
            new_config.set_username(username);
            new_config
        },
        Err(_) => Config::new(username, DEFAULT_MPW),
    };

    write_config(&config)
}

pub fn write_mpw(mpw: &str) -> Result<(), std::io::Error> {
    let config: Config = match read_config() {
        Ok(old_config) => {
            let mut new_config = old_config;
            new_config.set_mpw(mpw);
            new_config
        },
        Err(_) => Config::new(DEFAULT_USERNAME, mpw),
    };

    write_config(&config)
}

/// Deletes user config file.
pub fn delete_config() -> Result<(), std::io::Error>  {
    file::delete_file(&config_path())
}

#[test]
fn test_config_read_after_write() {
    let username: &str = "Test-name";
    let mpw: &str = "password123";
    write_username(username).expect("couldn't set username");
    write_mpw(mpw).expect("couldn't set mpw");
    let r_username: String = read_username();
    let r_mpw: String = read_mpw();
    assert_eq!(&username, &r_username);
    assert_eq!(&mpw, &r_mpw);
    delete_config().expect("couldn't delete config");
}

#[test]
fn test_config_read_empty() {
    let r_username: String = read_username();
    let r_mpw: String = read_mpw();
    assert_eq!(&r_username, DEFAULT_USERNAME);
    assert_eq!(&r_mpw, DEFAULT_MPW);
    delete_config().expect("couldn't delete config");
}
