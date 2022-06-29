#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#[macro_use] extern crate magic_crypt;
extern crate core;

// use std::io::Error;
use crate::database::chat::Chat;
use crate::database::group::Group;
use tauri::{Menu, MenuItem, Submenu, Window};
use tauri::plugin::Plugin;
use tauri::async_runtime;
use std::thread;

mod file;
mod database;
mod config;
mod encryption_unique_name;
// mod receiver;
mod encoding;
mod cmd;
mod interface;
use interface::send_message;

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
fn send_chat(group_id: i32, time: i64, message: String) {
  // QUESTION: can String be replaced by &str in the parameters?
  let name: String = config::read_username();
  let group: Group = find_group(group_id);
  let encodeddata: String = encoding::encode(&name, &group.decrypt_password(), &message);
  let serializeddata: String = encoding::group_encode(group.name, encodeddata);

  // TODO: encrypt message with user password
  let chat: Chat = Chat::new(group_id, time, &name, &message);

  // encoding::encode(&name, &group.encrypted_password, &message);
  send_message(serializeddata);
  database::save_chat(&chat);
}

fn find_group(group_id: i32) -> Group {
  let groups: Vec<Group> = database::get_groups();

  if groups.is_empty() {
    return Group::new(Some(0), "", "");
  }

  for group in &groups {
    if group.id == group_id {
      return group.clone();
    }
  }

  return Group::new(Some(0), "", "");
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
fn create_group(name: String, password: String) -> Group {
  // QUESTION: can String be replaced by &str in the parameters?
  let group: Group = Group::new(None, &name, &password);
  database::save_group(&group);
  group
}

#[tauri::command]
fn join_group(name: String, password: String) -> Group {
  // todo!("retrieve id");
  // todo!("retrieve bio");
  // todo!("retrieve password");
  // todo!("check password");

  let group: Group = Group::new(Some(0), &name, &password);
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
  let handle = async_runtime::spawn(async move {
    interface::start_client(window).await;
  });
  // let test = async_runtime::spawn(
  //   async move {
  //     interface::start_client(window);
  //   }
  // );
  // test.await.expect("TODO: panic message");
  // Plugin::initialize(interface::start_client(window));
  // thread::spawn(|| {
  //   interface::start_client(window);
  // });

}

#[tauri::command]
fn set_m_password(password: String) {
  config::write_mpw(&password);
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
