#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Menu, MenuItem, Submenu};

mod cmd;
mod serialize;

fn main() {
    let submenu = Submenu::new("MRC", Menu::new().add_native_item(MenuItem::Quit));
    let submenu2 = Submenu::new("Settings", Menu::new().add_native_item(MenuItem::Quit));
    let menu = Menu::new()
        .add_submenu(submenu)
        .add_submenu(submenu2);

    tauri::Builder::default()
        .menu(menu)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
