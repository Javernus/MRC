use crate::database::{chat, group};
use crate::database::group::Group;
use crate::database::chat::Chat;
use crate::file;

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


/// Returns all groups from database in vector format.
/// If groups are not found, an empty vector is returned.
///
/// returns: Vec<Group>
pub fn read_groups() -> Result<Vec<Group>, std::io::Error> {
    match file::read_file(&groups_path()) {
        Ok(contents) => match group::deserialize(&contents) {
            Ok(groups) => Ok(groups),
            Err(why) => Err(std::io::Error::from(why)),
        },
        Err(why) => Err(why),
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
pub fn read_chats(group_id: i32) -> Result<Vec<Chat>, std::io::Error> {
    match file::read_file(&chats_path(group_id)) {
        Ok(contents) => match chat::deserialize(&contents) {
            Ok(chats) => Ok(chats),
            Err(why) => Err(std::io::Error::from(why)),
        },
        Err(why) => Err(why),
    }
}

pub fn write_groups(groups: &Vec<Group>) -> Result<(), std::io::Error> {
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

pub fn write_chats(chats: &Vec<Chat>) -> Result<(), std::io::Error> {
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

pub fn delete_group() -> Result<(), std::io::Error> {
    file::delete_file(&groups_path())
}

pub fn delete_chat(group_id: i32) -> Result<(), std::io::Error> {
    file::delete_file(&chats_path(group_id))
}
