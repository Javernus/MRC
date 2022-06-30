use crate::config::config_struct::{Config, deserialize, serialize};
use crate::file;
use crate::hashing::hash_password;
use std::io::Error;

/// Returns string representation of path to config file.
///
/// returns: String
fn config_path() -> String {
    String::from("../data/config.json")
}

/// Reads config from database.
///
/// returns: Result<Config, std::io::Error>
pub fn read_config() -> Result<Config, Error> {
    match file::read_file(&config_path()) {
        Ok(contents) => match deserialize(&contents) {
            Ok(config) => Ok(config),
            Err(why) => Err(std::io::Error::from(why)),
        },
        Err(why) => Err(why),
    }
}

/// Writes config to database after hashing the password.
///
/// # Arguments
///
/// * `config`: the config to write to the database.
///
/// returns: Result<(), std::io::Error>
pub fn write_config(config: &Config) -> Result<(), Error> {
    let hashed_password: String = hash_password(&config.get_password());
    let mut hashed_config: Config = config.clone();
    hashed_config.set_password(&hashed_password);

    match serialize(&hashed_config) {
        Ok(serialized) => {
            match file::write_file(&config_path(), &serialized) {
                Ok(_) => Ok(()),
                Err(why) => Err(why),
            }
        }
        Err(why) => Err(std::io::Error::from(why)),
    }
}

/// Deletes user config file.
///
/// Returns: Result<(), std::io::Error>
#[allow(dead_code)]
pub fn delete_config() -> Result<(), Error> {
    file::delete_file(&config_path())
}

#[test]
fn test_config_io() {
    let config: Config = Config::new("Bob", "Bob's password");
    assert!(write_config(&config).is_ok());
    match read_config() {
        Ok(r_config) => {
            assert!(delete_config().is_ok());
            assert_eq!(&config.get_username(), &r_config.get_username());
            assert_ne!(&config.get_password(), &r_config.get_password());
        },
        Err(why) => panic!("failed to read config: {}", why),
    };
}
