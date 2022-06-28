use crate::file;
use crate::database::group::Group;
use crate::database::chat::Chat;

pub(crate) mod group;
pub(crate) mod chat;

/// Returns string representation of path to groups file in database.
/// Output: ../data/groups.json
///
/// returns: String
fn groups_path() -> String {
    String::from("../data/groups.json")
}

/// Returns string representation of path to chats file in database.
/// Output example: ../data/chats-42.json
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: String
fn chats_path(group_id: i32) -> String {
    format!("{}{}{}", "../data/chats-", group_id, ".json")
}

/// Appends group to groups.json in database.
///
/// # Arguments
///
/// * `g`: group to save.
pub fn save_group(g: &Group) {
    let groups_file: String = groups_path();
    let read_result = file::read_file(&groups_file);
    let groups: Vec<Group> = match read_result {
        Ok(contents) => {
            let mut current: Vec<Group> = group::deserialize(&contents);
            current.push(g.clone());
            current
        },
        Err(_) => {
            vec![g.clone()]
        },
    };

    let text: String = group::serialize(&groups);
    file::write_file(&groups_file, &text).expect("failed to save group");
}

/// Appends chat to chats-<<id>>.json in database.
///
/// # Arguments
///
/// * `c`: chat to save.
pub fn save_chat(c: &Chat) {
    let chats_file: String = chats_path(c.group_id);
    let read_result = file::read_file(&chats_file);
    let chats: Vec<Chat> = match read_result {
        Ok(contents) => {
            let mut current: Vec<Chat> = chat::deserialize(&contents);
            current.push(c.clone());
            current
        },
        Err(_) => {
            vec![c.clone()]
        },
    };

    let text: String = chat::serialize(&chats);
    file::write_file(&chats_file, &text).expect("failed to save chat");
}

/// Returns all groups from database in vector format.
/// If groups are not found, an empty vector is returned.
///
/// returns: Vec<Group>
pub fn get_groups() -> Vec<Group> {
    let groups_file: String = groups_path();
    let text = file::read_file(&groups_file);
    match text {
        Ok(contents) => {
            if contents.is_empty() {
                vec![]
            } else {
                group::deserialize(&contents)
            }
        },
        Err(_) => {
            vec![]
        },
    }
}

/// Returns all chats in group from database in vector format.
/// If chats are not found, an empty vector is returned.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: Vec<Chat>
pub fn get_chats(group_id: i32) -> Vec<Chat> {
    let chats_file: String = chats_path(group_id);
    let text = file::read_file(&chats_file);
    match text {
        Ok(contents) => {
            if contents.is_empty() {
                vec![]
            } else {
                chat::deserialize(&contents)
            }
        },
        Err(_) => {
            vec![]
        },
    }
}

/// Returns last chat in group from database.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: Chat
pub fn get_last_chat(group_id: i32) -> Chat {
    let chats: Vec<Chat> = get_chats(group_id);
    let mut last_chat: Chat = Chat::new(0, 0, "", "");
    for c in chats {
        if c.time > last_chat.time {
            last_chat = c;
        }
    }

    last_chat
}

/// Deletes chats file and group item in groups file from database.
///
/// # Arguments
///
/// * `group_id`: id of group.
pub fn delete_single_group(group_id: i32) {
    let filename: String = chats_path(group_id);
    file::delete_file(&filename).expect("failed to delete chats file");
    let groups_file: String = groups_path();
    let read_result = file::read_file(&groups_file);
    let groups: Vec<Group> = match read_result {
        Ok(contents) => {
            let mut current: Vec<Group> = group::deserialize(&contents);
            for i in 0..(current.len() - 1) {
                if current[i].id == group_id {
                    current.remove(i);
                }
            }

            current
        },
        Err(_) => {
            vec![]
        },
    };

    let text: String = group::serialize(&groups);
    file::write_file(&groups_file, &text).expect("failed to update groups file");
}

/// Deletes all chats and groups files from database.
pub fn delete_groups() {
    let groups: Vec<Group> = get_groups();
    for group in groups {
        let filename: String = chats_path(group.id);
        file::delete_file(&filename).expect("failed to delete chats file");
    }

    let groups_file: String = groups_path();
    file::delete_file(&groups_file).expect("failed to delete groups file");
}

#[test]
fn test_database() {
    let groups: Vec<Group> = vec![
        Group::new("Group", "password"),
        Group::new("People", "empty")
    ];

    for g in &groups {
        save_group(g);
    }

    let read_groups: Vec<Group> = get_groups();
    for i in 0..read_groups.len() {
        if read_groups[i].id == groups[0].id {
            assert_eq!(&groups[0], &read_groups[i]);
        } else if read_groups[i].id == groups[1].id {
            assert_eq!(&groups[1], &read_groups[i]);
        }
    }

    let chats_1: Vec<Chat> = vec![
        Chat::new(groups[0].id, 1000, "Alice", "Hi Bob!"),
        Chat::new(groups[0].id, 1200, "Bob", "Hi Alice!")
    ];

    for c in &chats_1 {
        save_chat(c);
    }

    let read_chats_1: Vec<Chat> = get_chats(groups[0].id);
    for i in 0..read_chats_1.len() {
        assert_eq!(&chats_1[i], &read_chats_1[i]);
    }

    let chats_2: Vec<Chat> = vec![
        Chat::new(groups[1].id, 4000, "Charlie", "Hi David!"),
        Chat::new(groups[1].id, 4200, "David", "Hi Charlie!")
    ];

    for c in &chats_2 {
        save_chat(c);
    }

    let read_chats_2: Vec<Chat> = get_chats(groups[1].id);
    for i in 0..read_chats_2.len() {
        assert_eq!(&chats_2[i], &read_chats_2[i]);
    }

    delete_groups();
}
