#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Menu, MenuItem, Submenu};
use crate::database::chat::Chat;
use crate::database::group::Group;

mod cmd;
mod database;

#[tauri::command]
fn get_groups() -> Vec<Group> {
  database::get_groups().into()
}

#[tauri::command]
fn send_chat(group_id: i32, time: i64, message: String) -> Chat {
  let chat: Chat = Chat::new(group_id, time, "TBImplemented", &message);
  database::save_chat(chat)
}

#[tauri::command]
fn remove_group(group_id: i32) {
  println!("This is where you remove the group: {}", group_id);
}

#[tauri::command]
fn create_group(name: String, bio: String, password: String) -> Group {
  let group: Group = Group::new(8, &name, &bio);
  database::save_group(group)
}

#[tauri::command]
fn get_newest_chat(group_id: i32) -> Chat {
  database::get_last_chat(group_id).into()
}

#[tauri::command]
fn get_chats(group_id: i32) -> Vec<Chat> {
  database::get_chats(group_id).into()
}

fn main() {
  let submenu: Submenu = Submenu::new("MRC", Menu::new().add_native_item(MenuItem::Quit));
  let submenu2: Submenu = Submenu::new("Settings", Menu::new().add_native_item(MenuItem::Quit));
  let menu: Menu = Menu::new()
    .add_submenu(submenu)
    .add_submenu(submenu2);

  tauri::Builder::default()
    .menu(menu)
    .invoke_handler(tauri::generate_handler![send_chat, get_chats, get_groups, get_newest_chat, remove_group, create_group])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
