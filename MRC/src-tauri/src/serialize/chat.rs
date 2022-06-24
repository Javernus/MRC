use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    id: u64,
    group_id: u64,
    time: u64,
    name: String,
    message: String,
}

impl Chat {
    pub fn new(id: u64, group_id: u64, time: u64, name: &str, message: &str) -> Chat {
        Chat {
            id,
            group_id,
            time,
            name: String::from(name),
            message: String::from(message),
        }
    }
}

pub fn serialize(chat: &Chat) -> String {
    serde_json::to_string(chat).unwrap()
}

pub fn deserialize(json: &str) -> Chat {
    serde_json::from_str(json).unwrap()
}

#[test]
fn test_create_chat() {
    let _chat = Chat::new(3, 1, 1000, "Bob", "message");
}

#[test]
fn test_ser_chat() {
    let chat: Chat = Chat::new(3, 1, 1000, "Bob", "message");
    let ser: String = serialize(&chat);

    assert_eq!(ser, "{\"id\":3,\"group_id\":1,\"time\":1000,\"name\":\"Bob\",\"message\":\"message\"}");
}

#[test]
fn test_deser_chat() {
    let chat: Chat = Chat::new(3, 1, 1000, "Bob", "message");
    let ser: String = serialize(&chat);
    let deser: Chat = deserialize(&ser);

    assert_eq!(chat.id, deser.id);
    assert_eq!(chat.group_id, deser.group_id);
    assert_eq!(chat.time, deser.time);
    assert_eq!(chat.name, deser.name);
    assert_eq!(chat.message, deser.message);
}
