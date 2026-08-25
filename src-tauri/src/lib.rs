mod commands;
mod engines;
mod error;
mod models;
mod secrets;
mod storage;

use models::AppState;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            connections: Mutex::new(Vec::new()),
        })
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            let loaded = storage::load(app.handle()).unwrap_or_default();
            *app.state::<AppState>().connections.lock().unwrap() = loaded;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_connections,
            commands::upsert_connection,
            commands::delete_connection,
            commands::test_connection,
            commands::fetch_server_info,
            commands::scan_keys,
            commands::get_entry,
            commands::set_entry,
            commands::delete_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
