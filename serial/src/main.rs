use serde::Serialize;
use serde::Deserialize;
use serde_json;
use chrono::DateTime;
use chrono::prelude::Utc;

fn format_time(time: DateTime<Utc>, fmt: &str) -> u64 {
    time.format(fmt).to_string().parse::<u64>().unwrap()
}

fn now() -> u64 {
    let now = Utc::now();
    let seconds = format_time(now, "%s");
    let milliseconds = format_time(now, "%3f");
    seconds * 1000 + milliseconds
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    message: String,
    name: String,
    time: u64,
}

impl Message {
    fn new(message: &str, name: &str) -> Message {
        Message {
            message: String::from(message),
            name: String::from(name),
            time: now(),
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
