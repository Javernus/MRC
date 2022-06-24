use crate::serialize::group::Group;
use crate::serialize::chat::Chat;

pub mod file;
pub mod group;
pub mod chat;

pub fn get_groups() -> Vec<Group> {
    // path: database/groups.json
    let filename: &str = "database/groups.json";
    let text: String = file::read_file(filename);
    let groups: Vec<Group> = group::deserialize(&text);
    groups
}

pub fn get_chats(group_id: u64) -> Vec<Chat> {
    // path: database/chats/<<group_id>>.json
    let filename: String = format!("{}{}{}", "database/chats/group-", group_id, ".json");
    let text: String = file::read_file(&filename);
    let chats: Vec<Chat> = chat::deserialize(&text);
    chats
}

pub fn save_groups(groups: &Vec<Group>) {
    // path: database/groups.json
    let filename: &str = "database/groups.json";
    let text: String = group::serialize(groups);
    file::write_file(filename, &text);
}

pub fn save_chats(chats: &Vec<Chat>) {
    // path: database/chats/<<group_id>>.json
    let filename: String = format!("{}{}{}", "database/chats/group-", chats[0].group_id, ".json");
    let text: String = chat::serialize(chats);
    file::write_file(&filename, &text);
}

#[test]
fn test_groups() {
    let group_1: Group = Group::new(1, "Group", "bio");
    let group_2: Group = Group::new(2, "People", "empty");
    let groups: Vec<Group> = vec![group_1, group_2];
    save_groups(&groups);
    let from_file: Vec<Group> = get_groups();

    for i in 0..2 {
        assert_eq!(groups[i].bio, from_file[i].bio);
        assert_eq!(groups[i].id, from_file[i].id);
        assert_eq!(groups[i].name, from_file[i].name);
    }
}

#[test]
fn test_chats() {
    let chat_1: Chat = Chat::new(1, 1, 1000, "Alice", "Hi Bob!");
    let chat_2: Chat = Chat::new(2, 1, 1200, "Bob", "Hi Alice!");
    let chats: Vec<Chat> = vec![chat_1, chat_2];
    save_chats(&chats);
    let from_file: Vec<Chat> = get_chats(1);

    for i in 0..2 {
        assert_eq!(chats[i].id, from_file[i].id);
        assert_eq!(chats[i].group_id, from_file[i].group_id);
        assert_eq!(chats[i].time, from_file[i].time);
        assert_eq!(chats[i].name, from_file[i].name);
        assert_eq!(chats[i].message, from_file[i].message);
    }
}
