use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub(crate) username: String,
}

impl User {
    pub(crate) fn new(username: &str) -> User {
        User {
            username: String::from(username),
        }
    }
}

pub const DEFAULT_USERNAME: &str = "Unnamed";

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
        serde_json::from_str(text).unwrap()
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
