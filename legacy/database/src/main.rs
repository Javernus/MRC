// copied from https://github.com/tauri-apps/tauri-plugin-sql#rust



use tauri_plugin_sql::TauriSql;

fn main() {
    tauri::Builder::default()
        .plugin(TauriSql::default())
        .build()
        .run();
}
