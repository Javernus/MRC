use crate::encryption_unique_name::{encrypt, decrypt};
use crate::config::get_mpw;
use serde::{Serialize, Deserialize};
use nanoid::nanoid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub bio: String,
    pub encrypted_password: String,
}

impl Group {
    /// Creates and returns new group. If no id is given, a unique id is generated.
    ///
    /// # Arguments
    ///
    /// * `id`: optional id of group.
    /// * `name`: name of group.
    /// * `bio`: bio of group.
    /// * `password`: password of group, use "" for empty password.
    ///
    /// returns: Group
    pub fn new(id: Option<i32>, name: &str, bio: &str, password: &str) -> Group {
        Group {
            id: {
                match id {
                    None => {
                        let a: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
                        nanoid!(9, &a).parse::<i32>().unwrap()
                    },
                    Some(g_id) => {
                        g_id
                    },
                }
            },
            name: String::from(name),
            bio: String::from(bio),
            encrypted_password: {
                if password.is_empty() {
                    password.to_string()
                } else {
                    encrypt(password, &get_mpw())
                }
            },
        }
    }

    pub fn decrypt_password(&self) -> String {
        decrypt(&*self.encrypted_password, &get_mpw())
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
        Group::new(Some(193), "Group", "bio", ""),
        Group::new(None, "People", "empty", "very strong password"),
    ];

    let ser: String = serialize(&groups);
    let deser: Vec<Group> = deserialize(&ser);

    for i in 0..2 {
        assert_eq!(groups[i], deser[i]);
    }
}

