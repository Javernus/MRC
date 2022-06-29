use crate::file;
use crate::config::user::User;
use crate::config::mpw::Mpw;

pub mod user;
pub mod mpw;

/// Returns string representation of path to username file.
/// Output: ../data/username.json
///
/// returns: String
fn username_path() -> String {
    String::from("../data/username.json")
}

/// Returns string representation of path to mpw file.
/// Output: ../data/mpw.json
///
/// returns: String
fn mpw_path() -> String {
    String::from("../data/mpw.json")
}

/// Sets username and saves it in config.
///
/// # Arguments
///
/// * `username`: username to set in config.
pub fn set_username(username: &str) {
    let user_file: String = username_path();
    let user: User = User::new(username);
    let text: String = user::serialize(&user);
    file::write_file(&user_file, &text).expect("failed to set username");
}

pub fn set_mpw(password: &str) {
    let mpw_file: String = username_path();
    let mpw: Mpw = Mpw::new(password);
    let text: String = mpw::serialize(&mpw);
    file::write_file(&mpw_file, &text).expect("failed to set mpw");
}

/// Retrieves username from config.
/// If no username is found, DEFAULT_USERNAME is set as username and gets returned as well.
///
/// returns: String
pub fn get_username() -> String {
    let user_file: String = username_path();
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

pub fn get_mpw() -> String {
    let mpw_file: String = mpw_path();
    match file::read_file(&mpw_file) {
        Ok(contents) => {
            if contents.is_empty() {
                set_mpw(mpw::DEFAULT_MPW);
                mpw::DEFAULT_MPW.to_string()
            } else {
                mpw::deserialize(&contents).mpw
            }
        },
        Err(_) => {
            set_mpw(mpw::DEFAULT_MPW);
            mpw::DEFAULT_MPW.to_string()
        },
    }
}

/// Deletes user config file.
pub fn delete_user() {
    let user_file: String = username_path();
    file::delete_file(&user_file).expect("failed to delete user file");
}

#[test]
fn test_set_username() {
    let username: &str = "Test-name";
    set_username(username);
    let read_username: String = get_username();
    assert_eq!(&username, &read_username);
    delete_user();

    let read_username: String = get_username();
    assert_eq!(&read_username, user::DEFAULT_USERNAME);
    delete_user();
}
