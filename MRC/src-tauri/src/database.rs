use crate::database::group::Group;
use crate::database::chat::Chat;

pub mod file;
pub mod group;
pub mod chat;

/// Returns string representation of path to groups file in database.
///
/// returns: String
fn groups_path() -> String {
    String::from("../database/groups.json")
}

/// Returns string representation of path to chats file in database.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: String
fn chats_path(group_id: i32) -> String {
    format!("{}{}{}", "../database/chats-", group_id, ".json")
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
pub fn save_group(g: Group) -> Group {
    let groups_file: String = groups_path();
    let mut current: Vec<Group> = group::deserialize(&file::read_file(&groups_file));
    current.push(g.clone());
    let text: String = group::serialize(&current);
    file::write_file(&groups_file, &text);
    g
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
pub fn save_chat(c: Chat) -> Chat {
    let chats_file: String = chats_path(c.group_id);
    // TODO: if chat file does not exist, create it and add empty vector to it.
    println!("{}", chats_file);
    let mut current: Vec<Chat> = chat::deserialize(&file::read_file(&chats_file));
    println!("{}", current.len());
    current.push(c.clone());
    println!("{}", current.len());
    let text: String = chat::serialize(&current);
    file::write_file(&chats_file, &text);
    c
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

pub fn get_last_chat(group_id: i32) -> Chat {
    let chats_file: String = chats_path(group_id);
    let text: String = file::read_file(&chats_file);
    let chats: Vec<Chat> = chat::deserialize(&text);
    // Find the chat with the highest time.
    let mut last_chat: Chat = Chat::new(0, 0, "", "");
    for c in chats {
        if c.time > last_chat.time {
            last_chat = c;
        }
    }
    last_chat
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
