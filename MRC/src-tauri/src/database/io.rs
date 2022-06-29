use std::io::Error;
use crate::database::{chat, group};
use crate::database::group::Group;
use crate::database::chat::Chat;
use crate::file;

/// Returns string representation of path to groups file in database.
///
/// returns: String
fn groups_path() -> String {
    String::from("../data/groups.json")
}

/// Returns string representation of path to chats file in database.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: String
fn chats_path(group_id: i32) -> String {
    format!("{}{}{}", "../data/chats-", group_id, ".json")
}


/// Returns all groups from database in vector format.
///
/// returns: Result<Vec<Group>, std::io::Error>
pub fn read_groups() -> Result<Vec<Group>, Error> {
    match file::read_file(&groups_path()) {
        Ok(contents) => match group::deserialize(&contents) {
            Ok(groups) => Ok(groups),
            Err(why) => Err(std::io::Error::from(why)),
        },
        Err(why) => Err(why),
    }
}

/// Returns all chats in group from database in vector format.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: Result<Vec<Chat>, std::io::Error>
pub fn read_chats(group_id: i32) -> Result<Vec<Chat>, Error> {
    match file::read_file(&chats_path(group_id)) {
        Ok(contents) => match chat::deserialize(&contents) {
            Ok(chats) => Ok(chats),
            Err(why) => Err(std::io::Error::from(why)),
        },
        Err(why) => Err(why),
    }
}

/// Writes groups to database in serialized vector format.
///
/// # Arguments
///
/// * `groups`: groups to write to database.
///
/// returns: Result<(), Error>
pub fn write_groups(groups: &Vec<Group>) -> Result<(), Error> {
    match group::serialize(&groups) {
        Ok(serialized) => {
            match file::write_file(&groups_path(), &serialized) {
                Ok(_) => Ok(()),
                Err(why) => Err(why),
            }
        }
        Err(why) => Err(std::io::Error::from(why)),
    }
}

/// Writes chats to database in serialized vector format.
///
/// # Arguments
///
/// * `chats`: chats to write to database.
///
/// returns: Result<(), Error>
pub fn write_chats(chats: &Vec<Chat>) -> Result<(), Error> {
    match chat::serialize(&chats) {
        Ok(serialized) => {
            match file::write_file(&chats_path(chats[0].get_group_id()), &serialized) {
                Ok(_) => Ok(()),
                Err(why) => Err(why),
            }
        }
        Err(why) => Err(std::io::Error::from(why)),
    }
}

/// Deletes groups file from the database.
///
/// returns: Result<(), std::io::Error>
#[allow(dead_code)]
pub fn delete_group() -> Result<(), Error> {
    file::delete_file(&groups_path())
}

/// Deletes a chats file from the database.
///
/// returns: Result<(), std::io::Error>
pub fn delete_chat(group_id: i32) -> Result<(), Error> {
    file::delete_file(&chats_path(group_id))
}

#[cfg(test)]
mod tests {
    use crate::database::io::{delete_chat, delete_group, write_chats, write_groups};
    use crate::{Chat, Group, read_chats, read_groups};

    #[test]
    fn test_chats_io() {
        let group_id = 123;
        let chats = vec![
            Chat::new(group_id, 1000, "name", "hello world"),
            Chat::new(group_id, 1000, "other", "good bye"),
        ];

        write_chats(&chats).expect("failed to write chats");
        let r_chats = read_chats(group_id).unwrap();

        assert!(delete_chat(group_id).is_ok());
        assert_eq!(&chats, &r_chats);
    }

    #[test]
    fn test_groups_io() {
        let groups: Vec<Group> = vec![
            Group::new(Some(345), "group1", "pass123"),
            Group::new(Some(543), "group2", "word234"),
        ];
        assert!(write_groups(&groups).is_ok());
        match read_groups() {
            Ok(r_groups) => {
                assert!(delete_group().is_ok());
                assert_eq!(&groups, &r_groups);
            },
            Err(why) => panic!("failed to read groups: {}", why),
        };
    }
}
