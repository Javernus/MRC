#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Menu, MenuItem, Submenu};
use tauri_plugin_sql::{Migration, MigrationKind, TauriSql};

mod cmd;

fn main() {
  let submenu = Submenu::new("MRC", Menu::new().add_native_item(MenuItem::Quit));
  let submenu2 = Submenu::new("Settings", Menu::new().add_native_item(MenuItem::Quit));
  let menu = Menu::new()
    .add_submenu(submenu)
    .add_submenu(submenu2);

  tauri::Builder::default()
    .menu(menu)
    .plugin(TauriSql::default().add_migrations(
      "sqlite:mrc.db",
      vec![Migration {
        version: 1,
        description: "Create Group and Chat tables",
        sql: include_str!("../migrations/tables.sql"),
        kind: MigrationKind::Up,
      }],
    ))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
