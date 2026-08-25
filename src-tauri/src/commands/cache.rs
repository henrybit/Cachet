use crate::commands::connections::require_connection;
use crate::engines;
use crate::error::AppError;
use crate::models::{AppState, CacheEntry, InfoItem, KeyPage};
use tauri::State;

#[tauri::command]
pub fn fetch_server_info(
    state: State<'_, AppState>,
    id: String,
    database: Option<u8>,
) -> Result<Vec<InfoItem>, AppError> {
    let config = require_connection(&state, &id)?;
    engines::fetch_server_info(&config, database)
}

#[tauri::command]
pub fn scan_keys(
    state: State<'_, AppState>,
    id: String,
    pattern: String,
    cursor: Option<String>,
    database: Option<u8>,
) -> Result<KeyPage, AppError> {
    let config = require_connection(&state, &id)?;
    engines::scan_keys(&config, &pattern, cursor.as_deref(), database)
}

#[tauri::command]
pub fn get_entry(
    state: State<'_, AppState>,
    id: String,
    key: String,
    database: Option<u8>,
) -> Result<CacheEntry, AppError> {
    let config = require_connection(&state, &id)?;
    engines::get_entry(&config, &key, database)
}

#[tauri::command]
pub fn set_entry(
    state: State<'_, AppState>,
    id: String,
    key: String,
    value: String,
    ttl_seconds: Option<u64>,
    database: Option<u8>,
) -> Result<(), AppError> {
    if key.trim().is_empty() {
        return Err(AppError::msg("键名不能为空"));
    }
    let config = require_connection(&state, &id)?;
    engines::set_entry(&config, key.trim(), &value, ttl_seconds, database)
}

#[tauri::command]
pub fn delete_entry(
    state: State<'_, AppState>,
    id: String,
    key: String,
    database: Option<u8>,
) -> Result<bool, AppError> {
    let config = require_connection(&state, &id)?;
    engines::delete_entry(&config, &key, database)
}
