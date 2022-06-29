use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Chat {
    group_id: i32,
    time: i64,
    name: String,
    message: String,
}

impl Chat {
    /// Creates and returns new chat.
    ///
    /// # Arguments
    ///
    /// * `group_id`: id of group.
    /// * `time`: time of chat.
    /// * `name`: name of sender.
    /// * `message`: message of chat.
    ///
    /// returns: Chat
    pub fn new(group_id: i32, time: i64, name: &str, message: &str) -> Chat {
        Chat {
            group_id,
            time,
            name: String::from(name),
            message: String::from(message),
        }
    }

    pub fn get_group_id(&self) -> i32 {
        self.group_id
    }

    pub fn get_time(&self) -> i64 {
        self.time
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_message(&self) -> String {
        self.message.to_string()
    }
}

/// Serializes vector of chats. Returns string in json format.
///
/// # Arguments
///
/// * `chats`: reference to vector of chats to serialize.
///
/// returns: String
pub fn serialize(chats: &Vec<Chat>) -> Result<String, serde_json::Error> {
    serde_json::to_string(chats)
}

/// Deserializes string to vector of chats. Returns vector of chats.
///
/// # Arguments
///
/// * `text`: reference to string to deserialize.
///
/// returns: Vec<Chat>
pub fn deserialize(text: &str) -> Result<Vec<Chat>, serde_json::Error> {
    if text.is_empty() {
        Ok(vec![])
    } else {
        serde_json::from_str(text)
    }
}

#[test]
fn test_chat() {
    let chats: Vec<Chat> = vec![
        Chat::new(1, 1000, "Alice", "Hi Bob!"),
        Chat::new(1, 1200, "Bob", "Hi Alice!"),
    ];

    let ser: String = serialize(&chats).unwrap();
    assert_eq!(ser, "[{\"group_id\":1,\"time\":1000,\"name\":\"Alice\",\"message\":\"Hi Bob!\"},{\"group_id\":1,\"time\":1200,\"name\":\"Bob\",\"message\":\"Hi Alice!\"}]");
    let deser: Vec<Chat> = deserialize(&ser).unwrap();

    for i in 0..2 {
        assert_eq!(chats[i], deser[i]);
    }
}
