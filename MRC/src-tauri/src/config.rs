use crate::file;
use crate::config::user::User;

pub(crate) mod user;

/// Returns string representation of path to configs file.
/// Output: ../config/user.json
///
/// returns: String
fn user_path() -> String {
    String::from("../database/config.json")
}

/// Sets username and saves it in config.
///
/// # Arguments
///
/// * `username`: username to set in config.
pub fn set_username(username: &str) {
    let user_file: String = user_path();
    let user: User = User::new(username);
    let text: String = user::serialize(&user);
    file::write_file(&user_file, &text).expect("failed to set username");
}

/// Retrieves username from config.
/// If no username is found, DEFAULT_USERNAME is set as username and gets returned as well.
///
/// returns: String
pub fn get_username() -> String {
    let user_file: String = user_path();
    match file::read_file(&user_file) {
        Ok(contents) => {
            if contents.is_empty() {
                set_username(user::DEFAULT_USERNAME);
                user::DEFAULT_USERNAME.to_string()
            } else {
                user::deserialize(&contents).username
            }
        },
        Err(_) => {
            set_username(user::DEFAULT_USERNAME);
            user::DEFAULT_USERNAME.to_string()
        },
    }
}

/// Deletes user config file.
pub fn delete_user() {
    let user_file: String = user_path();
    file::delete_file(&user_file).expect("failed to delete user file");
}

#[test]
fn test_set_username() {
    let username: &str = "Test-name";
    set_username(username);
    let read_username: String = get_username();

    dbg!(username, &read_username);
    assert_eq!(&username, &read_username);

    delete_user();
}

#[test]
fn test_get_empty_username() {
    let read_username: String = get_username();

    dbg!(&read_username);
    assert_eq!(&read_username, user::DEFAULT_USERNAME);

    delete_user();
}
