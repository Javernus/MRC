#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Menu, MenuItem, Submenu};

mod cmd;
mod serialize;

#[tauri::command]
fn get_groups() -> String {
  println!("get_groups");
  "[{ \"name\": \"MRC Alliance\", \"bio\": \"The official MRC chat for the MRC Alliance. Notify us of bugs, ask questions or give feedback here.\", \"id\": 1 },{ \"name\": \"Just shut up\", \"bio\": \"For the love of fuck.\", \"id\": 2 }]".into()
}

#[tauri::command]
fn send_chat(message: String, groupId: i32) {
  println!("This is where you get the message: {} in group {}", message, groupId);
}

fn main() {
  let submenu = Submenu::new("MRC", Menu::new().add_native_item(MenuItem::Quit));
  let submenu2 = Submenu::new("Settings", Menu::new().add_native_item(MenuItem::Quit));
  let menu = Menu::new()
    .add_submenu(submenu)
    .add_submenu(submenu2);

  tauri::Builder::default()
    .menu(menu)
    .invoke_handler(tauri::generate_handler![send_chat, get_groups])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
