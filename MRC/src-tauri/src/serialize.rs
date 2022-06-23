use serde_json;
use chrono::{DateTime, prelude::Utc};

mod group;
mod chat;

fn format_time(time: DateTime<Utc>, fmt: &str) -> u64 {
    time.format(fmt).to_string().parse::<u64>().unwrap()
}

fn now() -> u64 {
    let now = Utc::now();
    let seconds = format_time(now, "%s");
    let milliseconds = format_time(now, "%3f");
    seconds * 1000 + milliseconds
}


fn main() {
    let group = group::Group::new(2, "MRC Alliance", "bio");
    let ser_group = serde_json::to_string(&group).unwrap();
    let deser_group: group::Group = serde_json::from_str(&ser_group).unwrap();
    println!("{:?}", &group);
    println!("{}", &ser_group);
    println!("{:?}", &deser_group);

    let chat = chat::Chat::new(1, 2, 1000, "Bob", "MRC is great");
    let ser_chat = serde_json::to_string(&chat).unwrap();
    let deser_chat: chat::Chat = serde_json::from_str(&ser_chat).unwrap();
    println!("{:?}", &chat);
    println!("{}", &ser_chat);
    println!("{:?}", &deser_chat);
}
