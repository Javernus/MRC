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
/// returns: Config
pub fn deserialize(text: &str) -> User {
    serde_json::from_str(text).unwrap()
}

#[test]
fn test_config() {
    let user: User = User::new("Alice");
    let ser: String = serialize(&user);

    assert_eq!(ser, "{\"username\":\"Alice\"}");

    let deser: User = deserialize(&ser);

    assert_eq!(user, deser);
}
