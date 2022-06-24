use serde::{Serialize, Deserialize};
use std::io::{Read, Result, Write};
use std::fs;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    id: u64,
    name: String,
    bio: String,
}

impl Group {
    pub fn new(id: u64, name: &str, bio: &str) -> Group {
        Group {
            id,
            name: String::from(name),
            bio: String::from(bio),
        }
    }
}

pub fn serialize(group: &Group) -> String {
    serde_json::to_string(group).unwrap()
}

pub fn deserialize(json: &str) -> Group {
    serde_json::from_str(json).unwrap()
}

#[test]
fn test_create_group() {
    let _group = Group::new(1, "Group", "bio");
}

#[test]
fn test_ser_group() {
    let group: Group = Group::new(1, "Group", "bio");
    let ser: String = serialize(&group);

    assert_eq!(ser, "{\"id\":1,\"name\":\"Group\",\"bio\":\"bio\"}");
}

#[test]
fn test_deser_group() {
    let group: Group = Group::new(1, "Group", "bio");
    let ser: String = serialize(&group);
    let deser: Group = deserialize(&ser);

    assert_eq!(group.id, deser.id);
    assert_eq!(group.name, deser.name);
    assert_eq!(group.bio, deser.bio);
}
