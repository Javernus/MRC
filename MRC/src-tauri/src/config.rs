use crate::file;
use crate::config::user::User;
use std::io::Error;

pub(crate) mod user;

const DEFAULT_USERNAME: &str = "Unnamed";

/// Returns string representation of path to configs file.
/// Output: ../config/user.json
///
/// returns: String
fn user_path() -> String {
    String::from("../config/user.json")
}

/// Sets username and saves it in config.
///
/// # Arguments
///
/// * `username`: username to set in config.
///
/// returns: ()
pub fn set_username(username: &str) {
    let user_file: String = user_path();
    let user: User = User::new(username);
    let text: String = user::serialize(&user);
    file::write_file(&user_file, &text);
}

/// Retrieves username from config.
///
/// If no username is found, DEFAULT_USERNAME is set as username and gets returned as well.
///
/// returns: String
pub fn get_username() -> String {
    let user_file: String = user_path();
    let text: Result<String, Error> = file::read_file(&user_file);
    match text {
        Ok(contents) => {
            user::deserialize(&contents).username
        },
        Err(_) => {
            set_username(DEFAULT_USERNAME);
            DEFAULT_USERNAME.to_string()
        }
    }
}

/// Deletes user config file.
///
/// returns: ()
pub fn delete_user() {
    let filename: String = user_path();
    file::delete_file(&filename);
}

#[test]
fn test_set_username() {
    let username = String::from("Alice");
    set_username(&username);
    let read_username = get_username();

    dbg!(&username);
    dbg!(&read_username);
    assert_eq!(&username, &read_username);

    delete_user();
}

#[test]
fn test_get_empty_username() {
    let read_username = get_username();

    dbg!(&read_username);
    assert_eq!(&read_username, DEFAULT_USERNAME);

    delete_user();
}
