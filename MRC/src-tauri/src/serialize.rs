use crate::serialize::group::Group;
use crate::serialize::chat::Chat;

pub mod file;
pub mod group;
pub mod chat;

fn groups_path() -> String {
    String::from("database/groups.json")
}

fn chats_path(group_id: u64) -> String {
    format!("{}{}{}", "database/chats-", group_id, ".json")
}

pub fn save_groups(groups: &Vec<Group>) {
    let filename: String = groups_path();
    let text: String = group::serialize(groups);
    file::write_file(&filename, &text);
}

pub fn save_chats(chats: &Vec<Chat>) {
    let filename: String = chats_path(chats[0].group_id);
    let text: String = chat::serialize(chats);
    file::write_file(&filename, &text);
}

pub fn get_groups() -> Vec<Group> {
    let filename: String = groups_path();
    let text: String = file::read_file(&filename);
    let groups: Vec<Group> = group::deserialize(&text);
    groups
}

pub fn get_chats(group_id: u64) -> Vec<Chat> {
    let filename: String = chats_path(group_id);
    let text: String = file::read_file(&filename);
    let chats: Vec<Chat> = chat::deserialize(&text);
    chats
}

pub fn delete_chats(group_id: u64) {
    let filename: String = chats_path(group_id);
    file::delete_file(&filename);
}

pub fn delete_groups(group_ids: Vec<u64>) {
    for group_id in group_ids {
        delete_chats(group_id);
    }

    let filename: String = groups_path();
    file::delete_file(&filename);
}

#[test]
fn test_groups() {
    let group_1: Group = Group::new(1, "Group", "bio");
    let group_2: Group = Group::new(2, "People", "empty");
    let groups: Vec<Group> = vec![group_1, group_2];
    save_groups(&groups);
    let from_file: Vec<Group> = get_groups();

    for i in 0..2 {
        assert_eq!(groups[i], from_file[i]);
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
        assert_eq!(chats[i], from_file[i]);
    }
}

#[test]
fn test_delete() {
    save_groups(&vec![
        Group::new(1, "Group", "bio"),
        Group::new(2, "People", "empty")]);

    save_chats(&vec![
        Chat::new(1, 1, 1000, "Alice", "Hi Bob!"),
        Chat::new(2, 1, 1200, "Bob", "Hi Alice!")
    ]);

    save_chats(&vec![
        Chat::new(1, 2, 4000, "Charlie", "Hi David!"),
        Chat::new(2, 2, 4200, "David", "Hi Charlie!")
    ]);

    delete_groups(vec![1, 2]);
}