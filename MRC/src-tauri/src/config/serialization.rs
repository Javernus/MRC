use serde::{Serialize, Deserialize};

pub const DEFAULT_USERNAME: &str = "";
pub const DEFAULT_PASSWORD: &str = "";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    username: String,
    password: String,
}

impl Config {
    pub fn new(username: &str, password: &str) -> Config {
        Config {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }

    pub fn set_password(&mut self, password: &str) {
        self.password = password.to_string();
    }

    pub fn get_username(&self) -> String {
        self.username.to_string()
    }

    pub fn get_password(&self) -> String {
        self.password.to_string()
    }
}

pub fn serialize(config: &Config) -> Result<String, serde_json::Error> {
    serde_json::to_string(config)
}

pub fn deserialize(text: &str) -> Result<Config, serde_json::Error> {
    if text.is_empty() {
        Ok(Config::new(DEFAULT_USERNAME, DEFAULT_PASSWORD))
    } else {
        serde_json::from_str(text)
    }
}

#[test]
fn test_config_serialization() {
    let config: Config = Config::new("Alice", "password123");
    let ser: String = serialize(&config).unwrap();
    assert_eq!(ser, "{\"username\":\"Alice\",\"password\":\"password123\"}");
    let deser: Config = deserialize(&ser).unwrap();
    assert_eq!(config, deser);
}
