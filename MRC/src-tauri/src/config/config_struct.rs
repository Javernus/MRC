use serde::{Serialize, Deserialize};

pub const DEFAULT_USERNAME: &str = "default-username";
pub const DEFAULT_PASSWORD: &str = "default-password";
pub const EMPTY_PASSWORD: &str = "";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    username: String,
    password: String,
}

impl Config {
    /// Returns new Config object.
    ///
    /// # Arguments
    ///
    /// * `username`: username of account.
    /// * `password`: password of account.
    ///
    /// returns: Config
    pub fn new(username: &str, password: &str) -> Config {
        Config {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    /// Changes username.
    ///
    /// # Arguments
    ///
    /// * `username`: new username.
    pub fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }

    /// Changes password.
    ///
    /// # Arguments
    ///
    /// * `password`: new password.
    pub fn set_password(&mut self, password: &str) {
        self.password = password.to_string();
    }

    /// Returns username.
    pub fn get_username(&self) -> String {
        self.username.to_string()
    }

    /// Returns password.
    pub fn get_password(&self) -> String {
        self.password.to_string()
    }
}

/// Serializes config object to json string.
///
/// # Arguments
///
/// * `config`: object to serialize.
///
/// returns: Result<String, Error>
pub fn serialize(config: &Config) -> Result<String, serde_json::Error> {
    serde_json::to_string(config)
}

/// Deserializes json string to config object.
///
/// # Arguments
///
/// * `text`: text to deserialize.
///
/// returns: Result<Config, Error>
pub fn deserialize(text: &str) -> Result<Config, serde_json::Error> {
    if text.is_empty() {
        Ok(Config::new(DEFAULT_USERNAME, DEFAULT_PASSWORD))
    } else {
        serde_json::from_str(text)
    }
}

#[test]
fn test_config_struct() {
    let config: Config = Config::new("Alice", "password123");
    let ser: String = serialize(&config).unwrap();
    assert_eq!(ser, "{\"username\":\"Alice\",\"password\":\"password123\"}");
    let deser: Config = deserialize(&ser).unwrap();
    assert_eq!(config, deser);
}
