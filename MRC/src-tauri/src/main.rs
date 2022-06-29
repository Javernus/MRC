#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#[allow(non_snake_case)]

extern crate magic_crypt;
extern crate core;

use tauri::{Menu, MenuItem, Submenu, Window};
use tauri::async_runtime;

#[allow(unused_imports)]
use tauri::plugin::Plugin;
#[allow(unused_imports)]
use std::thread;

mod file;
mod database;
mod config;
mod encryption;
mod hashing;
mod encoding;
mod cmd;
mod interface;
use interface::send_message;
use database::chat::Chat;
use database::group::Group;
use database::io::{read_chats, read_groups};

/// Returns groups in vector format.
///
/// returns: Vec<Group>
#[tauri::command]
fn get_groups() -> Vec<Group> {
  match read_groups() {
    Ok(groups) => groups,
    Err(_) => vec![],
  }
}

/// Sends chat to group and saves it to the database.
///
/// # Arguments
///
/// * `group_id`: id of the group to send the chat to.
/// * `time`: the time the chat was sent.
/// * `message`: the message to send.
///
/// returns: Chat
#[tauri::command]
fn send_chat(group_id: i32, time: i64, message: String) {
  // QUESTION: can String be replaced by &str in the parameters?
  let name: String = config::read_username();
  let group: Group = find_group(group_id);
  let encodeddata: String = encoding::encode(&name, &group.get_decrypted_password(), &message);
  let serializeddata: String = encoding::group_encode(group.get_name(), encodeddata);

  // TODO: encrypt message with user password
  let chat: Chat = Chat::new(group_id, time, &name, &message);

  // encoding::encode(&name, &group.encrypted_password, &message);
  send_message(serializeddata);
  match database::append_chat(&chat) {
    Ok(_) => {}
    Err(_) => {}
  };
}

// Finds group based on group id.
fn find_group(group_id: i32) -> Group {
  let groups: Vec<Group> = match read_groups() {
    Ok(g) => g,
    Err(_) => vec![],
  };

  if groups.is_empty() {
    return Group::new(Some(0), "", "");
  }

  for group in &groups {
    if group.get_id() == group_id {
      return group.clone();
    }
  }

  return Group::new(Some(0), "", "");
}

#[allow(dead_code)]
fn find_group_ids(serializeddata: String) -> Vec<Group> {
  let groupdata: (String, String) = encoding::get_group(serializeddata);
  let groups: Vec<Group> = match read_groups() {
    Ok(g) => g,
    Err(_) => vec![],
  };
  let mut groupvec = Vec::new();

  if groups.is_empty() {
    return groupvec;
  }

  for group in &groups {
    if group.get_name() == groupdata.0 {
      groupvec.push(group.clone());
    }
  }

  return groupvec;

}
/// Removes group and all its chats from database.
///
/// # Arguments
///
/// * `group_id`: id of the group to remove.
#[tauri::command]
fn remove_group(group_id: i32) {
  match database::delete_single_group(group_id) {
    Ok(_) => {}
    Err(_) => {}
  };
}

/// Creates and returns a new group.
///
/// # Arguments
///
/// * `name`: name of the group.
/// * `bio`: bio of the group.
/// * `_password`: password of the group (optional).
///
/// returns: Group
#[tauri::command]
fn create_group(name: String, password: String) -> Group {
  // QUESTION: can String be replaced by &str in the parameters?
  let group: Group = Group::new(None, &name, &password);
  match database::append_group(&group) {
    Ok(_) => group,
    Err(_) => group,
  }
}

#[tauri::command]
fn join_group(name: String, password: String) -> Group {
  // todo!("retrieve id");
  // todo!("retrieve bio");
  // todo!("retrieve password");
  // todo!("check password");

  let group: Group = Group::new(Some(0), &name, &password);
  match database::append_group(&group) {
    Ok(_) => group,
    Err(_) => group,
  }
}

/// Returns the newest chat in group.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: Chat
#[tauri::command]
fn get_newest_chat(group_id: i32) -> Chat {
  match database::read_last_chat(group_id) {
    Ok(chat) => chat,
    Err(_) => Chat::new(-1, 0, "", ""),
  }
}

/// Returns all chats in group in vector format.
///
/// # Arguments
///
/// * `group_id`: id of group.
///
/// returns: Vec<Chat, Global>
#[tauri::command]
fn get_chats(group_id: i32) -> Vec<Chat> {
  match read_chats(group_id) {
    Ok(chats) => chats,
    Err(_) => vec![],
  }
}

/// Sets username in config.
///
/// # Arguments
///
/// * `username`: username to set.
#[tauri::command]
fn set_username(username: String) {
  // QUESTION: can String be replaced by &str in the parameters?
  match config::write_username(&username) {
    Ok(_) => {}
    Err(_) => {}
  };
}

/// Returns username in config.
///
/// returns: String
#[tauri::command]
fn get_username() -> String {
  // QUESTION: can String be replaced by &str in the parameters?
  config::read_username()
}

#[tauri::command]
fn start_client(window: Window) {
  #[allow(unused_must_use)]
  #[allow(unused_variables)]
  let handle = async_runtime::spawn(async move {
    interface::start_client(window).await;
  });
}

#[tauri::command]
fn set_m_password(password: String) {
  // TODO: change function name to set_user_password().
  match config::write_password(&password) {
    Ok(_) => {}
    Err(_) => {}
  };
}

fn main() {
  let app = Menu::new()
    .add_native_item(MenuItem::Quit);

  let text = Menu::new()
    .add_native_item(MenuItem::Undo)
    .add_native_item(MenuItem::Redo)
    .add_native_item(MenuItem::Separator)
    .add_native_item(MenuItem::Copy)
    .add_native_item(MenuItem::Paste)
    .add_native_item(MenuItem::Cut)
    .add_native_item(MenuItem::Separator)
    .add_native_item(MenuItem::SelectAll);

  let menu: Menu = Menu::new()
    .add_submenu(Submenu::new("MRC", app))
    .add_submenu(Submenu::new("File", text));

  tauri::Builder::default()
    .menu(menu)
    .invoke_handler(tauri::generate_handler![set_username, get_username, send_chat, get_chats, get_groups, get_newest_chat, remove_group, create_group, join_group, start_client, set_m_password])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
