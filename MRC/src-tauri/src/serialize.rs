use crate::serialize::group::Group;
use crate::serialize::chat::Chat;

pub mod file;
pub mod group;
pub mod chat;

/// Returns string representation of path to groups file in database.
///
/// returns: String
fn groups_path() -> String {
    String::from("database/groups.json")
}

/// Returns string representation of path to chats file in database.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: String
fn chats_path(group_id: u64) -> String {
    format!("{}{}{}", "database/chats-", group_id, ".json")
}

/// Saves groups to database in json format.
///
/// # Arguments
///
/// * `groups`: reference to vector of groups to save.
///
/// returns: ()
pub fn save_groups(groups: &Vec<Group>) {
    let filename: String = groups_path();
    let text: String = group::serialize(groups);
    file::write_file(&filename, &text);
}

/// Saves chats to database in json format.
///
/// # Arguments
///
/// * `chats`: reference to vector of chats to save.
///
/// returns: ()
pub fn save_chats(chats: &Vec<Chat>) {
    let filename: String = chats_path(chats[0].group_id);
    let text: String = chat::serialize(chats);
    file::write_file(&filename, &text);
}

/// Retrieves groups from database. Returns vector of groups.
///
/// returns: Vec<Group>
pub fn get_groups() -> Vec<Group> {
    let filename: String = groups_path();
    let text: String = file::read_file(&filename);
    let groups: Vec<Group> = group::deserialize(&text);
    groups
}

/// Retrieves chats from database. Returns vector of chats.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: Vec<Chat>
pub fn get_chats(group_id: u64) -> Vec<Chat> {
    let filename: String = chats_path(group_id);
    let text: String = file::read_file(&filename);
    let chats: Vec<Chat> = chat::deserialize(&text);
    chats
}

/// Deletes chats file in database.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: ()
pub fn delete_chats(group_id: u64) {
    let filename: String = chats_path(group_id);
    file::delete_file(&filename);
}

/// Deletes all chats with given group_id and deletes groups.
///
/// # Arguments
///
/// * `group_ids`: vector of ids of all groups to delete.
///
/// returns: ()
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