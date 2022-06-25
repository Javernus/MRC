use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Group {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) bio: String,
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

pub fn serialize(groups: &Vec<Group>) -> String {
    serde_json::to_string(groups).unwrap()
}

pub fn deserialize(text: &str) -> Vec<Group> {
    serde_json::from_str(text).unwrap()
}

#[test]
fn test_ser_group() {
    let group_1: Group = Group::new(1, "Group", "bio");
    let group_2: Group = Group::new(2, "People", "empty");
    let groups: Vec<Group> = vec![group_1, group_2];
    let ser: String = serialize(&groups);

    assert_eq!(ser, "[{\"id\":1,\"name\":\"Group\",\"bio\":\"bio\"},{\"id\":2,\"name\":\"People\",\"bio\":\"empty\"}]");
}

#[test]
fn test_deser_group() {
    let group_1: Group = Group::new(1, "Group", "bio");
    let group_2: Group = Group::new(2, "People", "empty");
    let groups: Vec<Group> = vec![group_1, group_2];
    let ser: String = serialize(&groups);
    let deser: Vec<Group> = deserialize(&ser);

    for i in 0..2 {
        assert_eq!(groups[i], deser[i]);
    }
}
