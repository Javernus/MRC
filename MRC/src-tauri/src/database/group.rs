use crate::encryption::{encrypt, decrypt};
use crate::config::{read_password};
use nanoid::nanoid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Group {
    id: i32,
    name: String,
    encrypted_password: String,
}

impl Group {
    /// Creates and returns new group.
    ///
    /// # Arguments
    ///
    /// * `name`: name of group.
    /// * `password`: password of group, use "" for empty password.
    ///
    /// returns: Group
    pub fn new(id: Option<i32>, name: &str, password: &str) -> Group {
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
            encrypted_password: {
                if password.is_empty() {
                    password.to_string()
                } else {
                    encrypt(password, &read_password())
                }
            },
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_encrypted_password(&self) -> String {
        self.encrypted_password.to_string()
    }

    pub fn get_decrypted_password(&self) -> String {
        decrypt(&self.get_encrypted_password(), &read_password())
    }
}

/// Serializes vector of groups. Returns string in json format.
///
/// # Arguments
///
/// * `groups`: reference to vector of groups to serialize.
///
/// returns: String
pub fn serialize(groups: &Vec<Group>) -> Result<String, serde_json::Error> {
    serde_json::to_string(groups)
}

/// Deserializes string to vector of groups. Returns vector of groups.
///
/// # Arguments
///
/// * `text`: reference to string to deserialize.
///
/// returns: Vec<Group>
pub fn deserialize(text: &str) -> Result<Vec<Group>, serde_json::Error> {
    if text.is_empty() {
        Ok(vec![])
    } else {
        serde_json::from_str(text)
    }
}

#[test]
fn test_group() {
    let groups: Vec<Group> = vec![
        Group::new(Some(193), "Group", ""),
        Group::new(None, "People", "very strong password"),
    ];

    let ser: String = serialize(&groups).unwrap();
    let deser: Vec<Group> = deserialize(&ser).unwrap();

    for i in 0..2 {
        assert_eq!(groups[i], deser[i]);
    }

    assert_eq!(groups[1].get_decrypted_password(), "very strong password");
}
