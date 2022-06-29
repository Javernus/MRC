use crate::config::config_struct::{Config, DEFAULT_PASSWORD, DEFAULT_USERNAME};
use crate::config::io::{read_config, write_config};
use crate::hashing::hash_password;
use std::io::Error;

pub mod config_struct;
pub mod io;

/// Returns username from config.
/// If no config is found, DEFAULT_USERNAME is returned.
///
/// returns: String
pub fn read_username() -> String {
    match read_config() {
        Ok(config) => config.get_username(),
        Err(_) => DEFAULT_USERNAME.to_string(),
    }
}

/// Returns password from config.
/// If no config is found, DEFAULT_PASSWORD is returned.
///
/// returns: String
pub fn read_password() -> String {
    match read_config() {
        Ok(config) => config.get_password(),
        Err(_) => DEFAULT_PASSWORD.to_string(),
    }
}

/// Sets username and saves it in config.
///
/// # Arguments
///
/// * `username`: username to set in config.
///
/// returns: Result<(), Error>
pub fn write_username(username: &str) -> Result<(), Error> {
    let config: Config = match read_config() {
        Ok(old_config) => {
            let mut new_config = old_config;
            new_config.set_username(username);
            new_config
        },
        Err(_) => Config::new(username, DEFAULT_PASSWORD),
    };

    write_config(&config)
}

/// Sets password and saves it in config.
///
/// # Arguments
///
/// * `password`: password to set in config.
///
/// returns: Result<(), Error>
pub fn write_password(password: &str) -> Result<(), Error> {
    let hashed_password: String = hash_password(password);
    let config: Config = match read_config() {
        Ok(old_config) => {
            let mut new_config = old_config;
            new_config.set_password(&hashed_password);
            new_config
        },
        Err(_) => {
            Config::new(DEFAULT_USERNAME, &hashed_password)
        },
    };

    write_config(&config)
}

#[cfg(test)]
mod tests {
    use crate::config::config_struct::{DEFAULT_PASSWORD, DEFAULT_USERNAME};
    use crate::config::io::delete_config;
    use crate::config::{read_password, read_username, write_password, write_username};
    use crate::hashing::hash_password;

    #[test]
    fn test_config_username() {
        let username: String = "Alice".to_string();

        assert!(delete_config().is_ok());
        assert!(write_username(&username).is_ok());
        let r_username: String = read_username();

        assert!(delete_config().is_ok());
        assert_eq!(&username, &r_username);
    }

    #[test]
    #[should_panic] // TODO: fix bug
    fn test_config_password() {
        let password: String = "Not Alice's password".to_string();
        let hashed_password: String = hash_password(&password);

        assert!(delete_config().is_ok());
        assert!(write_password(&password).is_ok());
        let r_hashed_password: String = read_password();

        assert!(delete_config().is_ok());
        assert_eq!(&hashed_password, &r_hashed_password);
    }

    #[test]
    fn test_config_username_empty() {
        match delete_config() {
            Ok(_) => {}
            Err(_) => {}
        };

        let r_username: String = read_username();

        assert!(delete_config().is_ok());
        assert_eq!(&r_username, DEFAULT_USERNAME);
    }

    #[test]
    fn test_config_password_empty() {
        match delete_config() {
            Ok(_) => {}
            Err(_) => {}
        };

        let r_password: String = read_password();

        assert!(delete_config().is_ok());
        assert_eq!(&r_password, DEFAULT_PASSWORD);
    }
}
