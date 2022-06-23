use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    message: String,
    name: String,
}

impl Message {
    fn new(message: &str, name: &str) -> Message {
        Message {
            message: String::from(message),
            name: String::from(name),
        }
    }
}

fn serialize(message: &Message) -> String {
    serde_json::to_string(message).unwrap()
}

fn deserialize(serialized: &str) -> Message {
    serde_json::from_str(serialized).unwrap()
}

fn main() {
    let message = Message::new("hi", "Bob");
    let serialized = serialize(&message);
    let deserialized = deserialize(&serialized);

    println!("{:?}", &message);
    println!("{}", &serialized);
    println!("{:?}", &deserialized);
}
