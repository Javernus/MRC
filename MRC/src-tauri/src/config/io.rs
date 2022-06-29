use std::io::Error;
use crate::config::config_struct::{Config, deserialize, serialize};
use crate::file;

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

/// Writes config to database.
///
/// # Arguments
///
/// * `config`: the config to write to the database.
///
/// returns: Result<(), std::io::Error>
pub fn write_config(config: &Config) -> Result<(), Error> {
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

/// Deletes user config file.
///
/// Returns: Result<(), std::io::Error>
#[allow(dead_code)]
pub fn delete_config() -> Result<(), Error> {
    file::delete_file(&config_path())
}

#[test]
fn test_config_io() {
    match delete_config() {
        Ok(_) => {}
        Err(_) => {}
    };

    let config: Config = Config::new("user", "pass");
    write_config(&config).expect("failed to write config");
    let r_config = match read_config() {
        Ok(conf) => conf,
        Err(why) => panic!("failed to read config: {}", why),
    };

    delete_config().unwrap();

    assert_eq!(config, r_config);
}
