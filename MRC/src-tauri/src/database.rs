use crate::database::group::Group;
use crate::database::chat::Chat;

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
fn chats_path(group_id: i32) -> String {
    format!("{}{}{}", "database/chats-", group_id, ".json")
}

/// Saves groups to database in json format.
///
/// # Important
///
/// Any existing file will be overwritten.
///
/// # Arguments
///
/// * `groups`: reference to vector of groups to save.
///
/// returns: ()
pub fn save_groups(groups: &Vec<Group>) {
    let groups_file: String = groups_path();
    let text: String = group::serialize(groups);
    file::write_file(&groups_file, &text);
}

/// Saves chats to database in json format.
///
/// # Important
///
/// All chats have to be in the same group.
/// Any existing file will be overwritten.
///
/// # Arguments
///
/// * `chats`: reference to vector of chats to save.
///
/// returns: ()
pub fn save_chats(chats: &Vec<Chat>) {
    let chats_file: String = chats_path(chats[0].group_id);
    let text: String = chat::serialize(chats);
    file::write_file(&chats_file, &text);
}

/// Retrieves groups from database. Returns vector of groups.
///
/// returns: Vec<Group>
pub fn get_groups() -> Vec<Group> {
    let groups_file: String = groups_path();
    let text: String = file::read_file(&groups_file);
    group::deserialize(&text)
}

/// Retrieves chats from database. Returns vector of chats.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: Vec<Chat>
pub fn get_chats(group_id: i32) -> Vec<Chat> {
    let chats_file: String = chats_path(group_id);
    let text: String = file::read_file(&chats_file);
    chat::deserialize(&text)
}

/// Deletes chats file in database.
///
/// # Important
///
/// Group file isn't updated.
/// Run save_groups() with remaining groups.
/// Tip: when deleting multiple chats, it is efficient to run save_groups() at the end.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: ()
pub fn delete_chats(group_id: i32) {
    let filename: String = chats_path(group_id);
    file::delete_file(&filename);
}

/// Deletes all chats and groups.
///
/// # Important
///
/// Database is wiped completely.
///
/// returns: ()
pub fn delete_groups() {
    let groups: Vec<Group> = get_groups();
    for group in groups {
        delete_chats(group.id);
    }

    let groups_file: String = groups_path();
    file::delete_file(&groups_file);
}

#[test]
fn test_serialize() {
    let groups: Vec<Group> = vec![
        Group::new(1, "Group", "bio"),
        Group::new(2, "People", "empty")
    ];

    save_groups(&groups);
    let read_groups: Vec<Group> = get_groups();
    for i in 0..2 {
        assert_eq!(groups[i], read_groups[i]);
    }

    let chats_1: Vec<Chat> = vec![
        Chat::new(1, 1, 1000, "Alice", "Hi Bob!"),
        Chat::new(2, 1, 1200, "Bob", "Hi Alice!")
    ];

    save_chats(&chats_1);
    let read_chats_1: Vec<Chat> = get_chats(1);
    for i in 0..2 {
        assert_eq!(chats_1[i], read_chats_1[i]);
    }

    let chats_2: Vec<Chat> = vec![
        Chat::new(1, 2, 4000, "Charlie", "Hi David!"),
        Chat::new(2, 2, 4200, "David", "Hi Charlie!")
    ];

    save_chats(&chats_2);
    let read_chats_2: Vec<Chat> = get_chats(2);
    for i in 0..2 {
        assert_eq!(chats_2[i], read_chats_2[i]);
    }

    delete_groups();
}
