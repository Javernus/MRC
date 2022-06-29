use crate::config::serialization::{Config, DEFAULT_PASSWORD, DEFAULT_USERNAME};
use crate::config::io::{read_config, write_config};
use std::io::Error;

pub mod serialization;
pub mod io;

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

pub fn write_password(password: &str) -> Result<(), Error> {
    let config: Config = match read_config() {
        Ok(old_config) => {
            let mut new_config = old_config;
            new_config.set_password(password);
            new_config
        },
        Err(_) => Config::new(DEFAULT_USERNAME, password),
    };

    write_config(&config)
}

#[cfg(test)]
mod tests {
    use crate::config::serialization::{DEFAULT_PASSWORD, DEFAULT_USERNAME};
    use crate::config::io::delete_config;
    use crate::config::{read_password, read_username, write_password, write_username};

    #[test]
    fn test_config_read_after_write() {
        let username: &str = "Test-name";
        let password: &str = "password123";
        write_username(username).expect("couldn't set username");
        write_password(password).expect("couldn't set password");
        let r_username: String = read_username();
        let r_password: String = read_password();
        assert_eq!(&username, &r_username);
        assert_eq!(&password, &r_password);
        delete_config().expect("couldn't delete config");
    }

    #[test]
    fn test_config_read_empty() {
        let r_username: String = read_username();
        let r_password: String = read_password();
        assert_eq!(&r_username, DEFAULT_USERNAME);
        assert_eq!(&r_password, DEFAULT_PASSWORD);
        delete_config().expect("couldn't delete config");
    }
}
