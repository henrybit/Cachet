use crate::error::AppError;
use crate::models::{ConnectionConfig, LegacyStoredConnection, StoredConnection};
use crate::secrets;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

const FILE_NAME: &str = "connections.json";

fn store_path(app: &AppHandle) -> Result<PathBuf, AppError> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|error| AppError::msg(error.to_string()))?;
    fs::create_dir_all(&dir)?;
    Ok(dir.join(FILE_NAME))
}

pub fn load(app: &AppHandle) -> Result<Vec<ConnectionConfig>, AppError> {
    let path = store_path(app)?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = fs::read_to_string(path)?;
    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }

    let legacy: Vec<LegacyStoredConnection> = serde_json::from_str(&raw)?;
    let mut migrated = false;
    let mut connections = Vec::with_capacity(legacy.len());

    for item in legacy {
        let legacy_password = item
            .password
            .clone()
            .filter(|value: &String| !value.is_empty());
        let stored = item.into_stored();
        let password = match legacy_password {
            Some(password) => {
                secrets::set_password(&stored.id, &password)?;
                migrated = true;
                Some(password)
            }
            None => secrets::get_password(&stored.id)?,
        };
        connections.push(stored.into_config(password));
    }

    if migrated {
        save(app, &connections)?;
    }

    Ok(connections)
}

pub fn save(app: &AppHandle, connections: &[ConnectionConfig]) -> Result<(), AppError> {
    let path = store_path(app)?;
    let stored: Vec<StoredConnection> = connections.iter().map(StoredConnection::from).collect();
    let raw = serde_json::to_string_pretty(&stored)?;
    fs::write(path, raw)?;
    Ok(())
}
