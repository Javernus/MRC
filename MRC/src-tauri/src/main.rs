#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate core;

use crate::database::chat::Chat;
use crate::database::group::Group;
use tauri::{Menu, MenuItem, Submenu};
use std::thread;
use std::time::Duration;

pub(crate) mod file;
mod database;
mod config;
mod cmd;

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
  let group: Group = Group::new(8, &name, &bio);
  database::save_group(&group);
  group
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

fn main() {
  let submenu: Submenu = Submenu::new("MRC", Menu::new().add_native_item(MenuItem::Quit));
  let submenu2: Submenu = Submenu::new("Settings", Menu::new().add_native_item(MenuItem::Quit));
  let menu: Menu = Menu::new()
    .add_submenu(submenu)
    .add_submenu(submenu2);

  thread::spawn(|| {
    loop {
      thread::sleep(Duration::from_secs(1));
      println!("Test");
    }
  });

  tauri::Builder::default()
    .menu(menu)
    .invoke_handler(tauri::generate_handler![set_username, get_username, send_chat, get_chats, get_groups, get_newest_chat, remove_group, create_group])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
