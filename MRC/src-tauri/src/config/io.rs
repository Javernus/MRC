use crate::config::serialization::{Config, deserialize, serialize};
use crate::file;

/// Returns string representation of path to config file.
/// Output: ../data/config.json
///
/// returns: String
fn config_path() -> String {
    String::from("../data/config.json")
}

pub fn read_config() -> Result<Config, std::io::Error> {
    match file::read_file(&config_path()) {
        Ok(contents) => match deserialize(&contents) {
            Ok(config) => Ok(config),
            Err(why) => Err(std::io::Error::from(why)),
        },
        Err(why) => Err(why),
    }
}

pub fn write_config(config: &Config) -> Result<(), std::io::Error> {
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
pub fn delete_config() -> Result<(), std::io::Error> {
    file::delete_file(&config_path())
}
