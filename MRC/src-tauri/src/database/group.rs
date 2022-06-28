use serde::{Serialize, Deserialize};
use nanoid::nanoid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Group {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) bio: String,
}

impl Group {
    /// Creates and returns new group.
    ///
    /// # Arguments
    ///
    /// * `name`: name of group.
    /// * `bio`: bio of group.
    ///
    /// returns: Group
    pub fn new(name: &str, bio: &str) -> Group {
        let alphabet: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let id: i32 = nanoid!(9, &alphabet).parse().unwrap();

        Group {
            id,
            name: String::from(name),
            bio: String::from(bio),
        }
    }

    pub fn init(id: i32, name: &str, bio: &str) -> Group {
        Group {
            id,
            name: String::from(name),
            bio: String::from(bio),
        }
    }
}

/// Serializes vector of groups. Returns string in json format.
///
/// # Arguments
///
/// * `groups`: reference to vector of groups to serialize.
///
/// returns: String
pub fn serialize(groups: &Vec<Group>) -> String {
    serde_json::to_string(groups).unwrap()
}

/// Deserializes string to vector of groups. Returns vector of groups.
///
/// # Arguments
///
/// * `text`: reference to string to deserialize.
///
/// returns: Vec<Group>
pub fn deserialize(text: &str) -> Vec<Group> {
    if text.is_empty() {
        vec![]
    } else {
        serde_json::from_str(text).unwrap()
    }
}

#[test]
fn test_group() {
    let groups: Vec<Group> = vec![
        Group::new("Group", "bio"),
        Group::new("People", "empty"),
    ];

    let ser: String = serialize(&groups);
    let deser: Vec<Group> = deserialize(&ser);

    for i in 0..2 {
        assert_eq!(groups[i], deser[i]);
    }
}
