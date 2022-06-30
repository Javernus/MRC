use crate::config::config_struct::{Config, DEFAULT_PASSWORD, DEFAULT_USERNAME, EMPTY_PASSWORD};
use crate::config::io::{read_config, write_config};
use crate::hashing::{verify_password};
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

/// Returns hashed password from config in database.
/// If no config is found, DEFAULT_PASSWORD is returned.
///
/// returns: String
pub fn read_hashed_password() -> String {
    match read_config() {
        Ok(config) => config.get_password(),
        Err(_) => EMPTY_PASSWORD.to_string(),
    }
}

/// Checks if password is set in config file.
///
/// returns: bool
pub fn is_password_set() -> bool {
    read_hashed_password() != EMPTY_PASSWORD.to_string()
}

/// Verifies the given user password against the hashed password in the config file.
///
/// # Arguments
///
/// * `user_password`: user password to verify.
///
/// returns: bool
pub fn verify_user_password(user_password: &str) -> bool {
    is_password_set() && verify_password(user_password, &read_hashed_password())
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

/// Sets password and saves the hashed version in config.
///
/// # Arguments
///
/// * `password`: password to set in config.
///
/// returns: Result<(), Error>
pub fn write_password(password: &str) -> Result<(), Error> {
    let config: Config = match read_config() {
        Ok(old_config) => {
            let mut new_config = old_config;
            new_config.set_password(&password);
            new_config
        },
        Err(_) => {
            Config::new(DEFAULT_USERNAME, password)
        },
    };

    write_config(&config)
}

#[cfg(test)]
mod tests {
    use crate::config::config_struct::{DEFAULT_USERNAME, EMPTY_PASSWORD};
    use crate::config::io::delete_config;
    use crate::config::{is_password_set, read_hashed_password, read_username, verify_user_password, write_password, write_username};
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
    fn test_config_password() {
        let password: String = "Not Alice's password".to_string();
        let hashed_password: String = hash_password(&password);

        assert!(delete_config().is_ok());
        assert!(write_password(&password).is_ok());
        let r_hashed_password: String = read_hashed_password();

        assert!(delete_config().is_ok());
        assert_eq!(&hashed_password, &r_hashed_password);
    }

    #[test]
    fn test_config_user_password() {
        let password: String = "Not Alice's password".to_string();

        assert!(delete_config().is_ok());
        assert!(write_password(&password).is_ok());

        assert!(is_password_set());
        assert!(verify_user_password(&password));
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

        let r_password: String = read_hashed_password();

        assert!(delete_config().is_ok());
        assert_eq!(&r_password, EMPTY_PASSWORD);
    }

    #[test]
    fn test_config_user_password_empty() {
        match delete_config() {
            Ok(_) => {}
            Err(_) => {}
        };

        assert!(!is_password_set());
        assert_eq!(read_hashed_password(), EMPTY_PASSWORD);
        assert!(!verify_user_password(EMPTY_PASSWORD));
    }
}
