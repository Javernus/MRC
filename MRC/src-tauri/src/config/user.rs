use serde::{Serialize, Deserialize};

pub const DEFAULT_USERNAME: &str = "";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub username: String,
}

impl User {
    pub fn new(username: &str) -> User {
        User {
            username: username.to_string(),
        }
    }
}

/// Serializes user. Returns string in json format.
///
/// # Arguments
///
/// * `user`: reference to user to serialize.
///
/// returns: String
pub fn serialize(user: &User) -> String {
    serde_json::to_string(user).unwrap()
}

/// Deserializes string to user. Returns user.
///
/// # Arguments
///
/// * `text`: reference to string to deserialize.
///
/// returns: User
pub fn deserialize(text: &str) -> User {
    if text.is_empty() {
        User::new(DEFAULT_USERNAME)
    } else {
        match serde_json::from_str(text) {
            Ok(user) => user,
            Err(_) => User::new(DEFAULT_USERNAME),
        }
    }
}

#[test]
fn test_config() {
    let user: User = User::new("Alice");
    let ser: String = serialize(&user);
    assert_eq!(ser, "{\"username\":\"Alice\"}");
    let deser: User = deserialize(&ser);
    assert_eq!(user, deser);
}
