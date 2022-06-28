#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate core;

use crate::database::chat::Chat;
use crate::database::group::Group;
use tauri::{Menu, MenuItem, Submenu, Window, AboutMetadata};
use std::thread;

pub(crate) mod file;
mod database;
mod config;
mod cmd;
mod receiver;

/// Returns groups in vector format.
///
/// returns: Vec<Group>
#[tauri::command]
fn get_groups() -> Vec<Group> {
  database::get_groups().into()
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
fn send_chat(group_id: i32, time: i64, message: String) -> Chat {
  // QUESTION: can String be replaced by &str in the parameters?
  let name: String = config::get_username();
  let chat: Chat = Chat::new(group_id, time, &name, &message);
  database::save_chat(&chat);
  chat
}

/// Removes group and all its chats from database.
///
/// # Arguments
///
/// * `group_id`: id of the group to remove.
#[tauri::command]
fn remove_group(group_id: i32) {
  database::delete_single_group(group_id);
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
fn create_group(name: String, bio: String, password: String) -> Group {
  // QUESTION: can String be replaced by &str in the parameters?
  let group: Group = Group::new(&name, &bio);
  database::save_group(&group);
  group
}

#[tauri::command]
fn join_group(group: String, password: String) -> Group {
  Group::init(0, &group, "")
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
  database::get_last_chat(group_id).into()
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
  database::get_chats(group_id).into()
}

/// Sets username in config.
///
/// # Arguments
///
/// * `username`: username to set.
#[tauri::command]
fn set_username(username: String) {
  // QUESTION: can String be replaced by &str in the parameters?
  config::set_username(&username);
}

/// Returns username in config.
///
/// returns: String
#[tauri::command]
fn get_username() -> String {
  // QUESTION: can String be replaced by &str in the parameters?
  config::get_username()
}

#[tauri::command]
fn receiver(window: Window) {
  thread::spawn(|| {
    receiver::start_receiver(window);
  });
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
    .invoke_handler(tauri::generate_handler![set_username, get_username, send_chat, get_chats, get_groups, get_newest_chat, remove_group, create_group, join_group, receiver])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
