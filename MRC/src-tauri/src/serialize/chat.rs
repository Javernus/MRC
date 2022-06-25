use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Chat {
    pub(crate) id: u64,
    pub(crate) group_id: u64,
    pub(crate) time: u64,
    pub(crate) name: String,
    pub(crate) message: String,
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

pub fn serialize(chats: &Vec<Chat>) -> String {
    serde_json::to_string(chats).unwrap()
}

pub fn deserialize(text: &str) -> Vec<Chat> {
    serde_json::from_str(text).unwrap()
}

#[test]
fn test_ser_chat() {
    let chat_1: Chat = Chat::new(1, 1, 1000, "Alice", "Hi Bob!");
    let chat_2: Chat = Chat::new(2, 1, 1200, "Bob", "Hi Alice!");
    let chats: Vec<Chat> = vec![chat_1, chat_2];
    let ser: String = serialize(&chats);

    assert_eq!(ser, "[{\"id\":1,\"group_id\":1,\"time\":1000,\"name\":\"Alice\",\"message\":\"Hi Bob!\"},{\"id\":2,\"group_id\":1,\"time\":1200,\"name\":\"Bob\",\"message\":\"Hi Alice!\"}]");
}

#[test]
fn test_deser_chat() {
    let chat_1: Chat = Chat::new(1, 1, 1000, "Alice", "Hi Bob!");
    let chat_2: Chat = Chat::new(2, 1, 1200, "Bob", "Hi Alice!");
    let chats: Vec<Chat> = vec![chat_1, chat_2];
    let ser: String = serialize(&chats);
    let deser: Vec<Chat> = deserialize(&ser);

    for i in 0..2 {
        assert_eq!(chats[i], deser[i]);
    }
}
