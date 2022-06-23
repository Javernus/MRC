#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri_plugin_sql::{Migration, MigrationKind, TauriSql};

mod cmd;

fn main() {
  tauri::Builder::default()
    .plugin(TauriSql::default().add_migrations(
      "sqlite:test.db",
      vec![Migration {
        version: 1,
        description: "Create Group and Chat tables",
        sql: include_str!("../migrations/1.sql"),
        kind: MigrationKind::Up,
      }],
    ))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
