use crate::database::group::Group;
use crate::database::chat::Chat;
use crate::database::io::{read_chats, read_groups, write_chats, write_groups};
use std::io::Error;

pub mod group;
pub mod chat;
pub mod io;

/// Appends group to groups.json in database.
///
/// # Arguments
///
/// * `group`: group to save.
///
/// returns: Result<(), Error>
pub fn append_group(group: &Group) -> Result<(), Error> {
    let groups: Vec<Group> = match read_groups() {
        Ok(old_groups) => {
            let mut new_groups: Vec<Group> = old_groups;
            new_groups.push(group.clone());
            new_groups
        }
        Err(_) => {
            vec![group.clone()]
        },
    };

    write_groups(&groups)
}

/// Appends chat to chats-<<id>>.json in database.
///
/// # Arguments
///
/// * `chat`: chat to save.
///
/// returns: Result<(), Error>
pub fn append_chat(chat: &Chat) -> Result<(), Error> {
    let chats: Vec<Chat> = match read_chats(chat.get_group_id()) {
        Ok(old_chats) => {
            let mut new_chats: Vec<Chat> = old_chats;
            new_chats.push(chat.clone());
            new_chats
        }
        Err(_) => {
            vec![chat.clone()]
        },
    };

    write_chats(&chats)
}

/// Returns last chat in group from database.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: Result<Chat, Error>
pub fn read_last_chat(group_id: i32) -> Result<Chat, Error> {
    let mut last_chat: Chat = Chat::new(-1, 0, "", "");

    match read_chats(group_id) {
        Ok(chats) => {
            for chat in chats {
                if chat.get_time() > last_chat.get_time() {
                    last_chat = chat;
                }
            }

            Ok(last_chat)
        },
        Err(why) => Err(why),
    }
}

/// Deletes chats file and group item in groups file from database.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: Result<(), Error>
pub fn delete_single_group(group_id: i32) -> Result<(), Error> {
    match io::delete_chat(group_id) {
        Ok(_) => {},
        Err(why) => return Err(why),
    }

    let groups: Vec<Group> = match read_groups() {
        Ok(old_groups) => {
            let mut new_groups: Vec<Group> = old_groups.clone();

            for (i, group) in old_groups.iter().enumerate() {
                if group.get_id() == group_id {
                    new_groups.remove(i);
                }
            }

            new_groups
        },
        Err(_) => vec![],
    };

    write_groups(&groups)
}

/// Deletes all chats and groups files from database.
///
/// returns: Result<(), Error>
#[allow(dead_code)]
pub fn delete_groups() -> Result<(), Error> {
    match read_groups() {
        Ok(groups) => {
            for group in groups {
                match io::delete_chat(group.get_id()) {
                    Ok(_) => {}
                    Err(why) => return Err(why),
                };
            }

            io::delete_group()
        },
        Err(why) => Err(why),
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::io::Error;
    use crate::{Chat, Group, read_chats, read_groups};
    use crate::database::{append_chat, append_group, delete_groups, delete_single_group};

    #[test]
    fn test_database_groups() {
        match delete_groups() {
            Ok(_) => {}
            Err(_) => {}
        };

        let groups: Vec<Group> = vec![
            Group::new(Some(192), "Group", ""),
            Group::new(None, "People", "very strong password")
        ];

        for g in &groups {
            assert!(append_group(g).is_ok());
        }

        let read_groups: Vec<Group> = match read_groups() {
            Ok(g) => g,
            Err(why) => panic!("failed to read groups: {}", why),
        };

        assert!(delete_groups().is_ok());

        for i in 0..read_groups.len() {
            if read_groups[i].get_id() == groups[0].get_id() {
                assert_eq!(&groups[0], &read_groups[i]);
            } else if read_groups[i].get_id() == groups[1].get_id() {
                assert_eq!(&groups[1], &read_groups[i]);
            }
        }
    }

    #[test]
    fn test_database_chats_1() {
        match delete_groups() {
            Ok(_) => {}
            Err(_) => {}
        };

        let group_id: i32 = 192;
        let group: Group = Group::new(Some(group_id), "Group", "");
        let chats: Vec<Chat> = vec![
            Chat::new(group_id, 1000, "Alice", "Hi Bob!"),
            Chat::new(group_id, 1200, "Bob", "Hi Alice!")
        ];

        assert!(append_group(&group).is_ok());

        for c in &chats {
            assert!(append_chat(c).is_ok());
        }

        let r_chats: Vec<Chat> = match read_chats(group_id) {
            Ok(c) => c,
            Err(why) => panic!("failed to read chats: {}", why),
        };

        assert!(delete_single_group(group_id).is_ok());

        for i in 0..r_chats.len() {
            assert_eq!(&chats[i], &r_chats[i]);
        }
    }

    #[test]
    fn test_database_chats_2() {
        match delete_groups() {
            Ok(_) => {}
            Err(_) => {}
        };

        let group: Group = Group::new(None, "People", "very strong password");
        let group_id = group.get_id();
        let chats: Vec<Chat> = vec![
            Chat::new(group_id, 4000, "Charlie", "Hi David!"),
            Chat::new(group_id, 4200, "David", "Hi Charlie!")
        ];

        assert!(append_group(&group).is_ok());

        for c in &chats {
            assert!(append_chat(c).is_ok());
        }

        let r_chats: Vec<Chat> = match read_chats(group_id) {
            Ok(c) => c,
            Err(why) => panic!("failed to read chats: {}", why),
        };

        assert!(delete_single_group(group_id).is_ok());

        for i in 0..r_chats.len() {
            assert_eq!(&chats[i], &r_chats[i]);
        }
    }
}
