// copied from https://github.com/tauri-apps/tauri-plugin-sql#webview



import Database from 'tauri-plugin-sql-api'

// sqlite. The path is relative to `tauri::api::path::BaseDirectory::App`.
const db = await Database.load('sqlite:test.db')
// mysql
const db = await Database.load('mysql://user:pass@host/database')
// postgres
const db = await Database.load('postgres://postgres:password@localhost/test')

await db.execute('INSERT INTO ...')
