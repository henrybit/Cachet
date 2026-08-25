use crate::engines;
use crate::error::AppError;
use crate::models::{AppState, ConnectionConfig, ConnectionDraft, ConnectionPublic};
use crate::secrets;
use crate::storage;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, State};

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or_default()
}

fn find_connection<'a>(
    connections: &'a [ConnectionConfig],
    id: &str,
) -> Result<&'a ConnectionConfig, AppError> {
    connections
        .iter()
        .find(|item| item.id == id)
        .ok_or_else(|| AppError::msg("连接不存在"))
}

fn resolve_password(
    connection_id: &str,
    draft_password: Option<&str>,
    existing_password: Option<&str>,
) -> Result<Option<String>, AppError> {
    match draft_password.filter(|value| !value.is_empty()) {
        Some(password) => {
            secrets::set_password(connection_id, password)?;
            Ok(Some(password.to_string()))
        }
        None => Ok(existing_password.map(str::to_string)),
    }
}

#[tauri::command]
pub fn list_connections(state: State<'_, AppState>) -> Result<Vec<ConnectionPublic>, AppError> {
    let connections = state.connections.lock().unwrap();
    Ok(connections
        .iter()
        .map(ConnectionPublic::from)
        .collect())
}

#[tauri::command]
pub fn upsert_connection(
    app: AppHandle,
    state: State<'_, AppState>,
    draft: ConnectionDraft,
) -> Result<ConnectionPublic, AppError> {
    draft.validate().map_err(AppError::msg)?;
    let mut connections = state.connections.lock().unwrap();
    let timestamp = now_ms();
    let existing = draft
        .id
        .as_ref()
        .and_then(|id| connections.iter().find(|item| item.id == *id).cloned());

    let connection_id = existing
        .as_ref()
        .map(|item| item.id.clone())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let password = resolve_password(
        &connection_id,
        draft.password.as_deref(),
        existing.as_ref().and_then(|item| item.password.as_deref()),
    )?;

    let saved = ConnectionConfig {
        id: connection_id,
        name: draft.name.trim().to_string(),
        engine: draft.engine,
        host: draft.host.trim().to_string(),
        port: draft.port,
        username: draft
            .username
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty()),
        password,
        database: draft.database,
        tls: draft.tls,
        timeout_ms: draft.timeout_ms.max(500),
        created_at: existing
            .as_ref()
            .map(|item| item.created_at)
            .unwrap_or(timestamp),
        updated_at: timestamp,
    };

    if let Some(index) = connections.iter().position(|item| item.id == saved.id) {
        connections[index] = saved.clone();
    } else {
        connections.push(saved.clone());
    }
    connections.sort_by(|left, right| left.name.to_lowercase().cmp(&right.name.to_lowercase()));
    storage::save(&app, &connections)?;
    Ok(ConnectionPublic::from(&saved))
}

#[tauri::command]
pub fn delete_connection(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let mut connections = state.connections.lock().unwrap();
    let before = connections.len();
    connections.retain(|item| item.id != id);
    if connections.len() == before {
        return Err(AppError::msg("连接不存在"));
    }
    secrets::delete_password(&id)?;
    storage::save(&app, &connections)?;
    Ok(())
}

#[tauri::command]
pub fn test_connection(
    state: State<'_, AppState>,
    draft: ConnectionDraft,
) -> Result<String, AppError> {
    draft.validate().map_err(AppError::msg)?;
    let existing = draft.id.as_ref().and_then(|id| {
        state
            .connections
            .lock()
            .ok()
            .and_then(|connections| connections.iter().find(|item| item.id == *id).cloned())
    });

    let connection_id = draft
        .id
        .clone()
        .unwrap_or_else(|| "test-connection".to_string());

    let password = match draft.password.as_deref().filter(|value| !value.is_empty()) {
        Some(password) => Some(password.to_string()),
        None => existing
            .as_ref()
            .and_then(|item| item.password.clone())
            .or_else(|| secrets::get_password(&connection_id).ok().flatten())
            .filter(|value| !value.is_empty()),
    };

    let config = ConnectionConfig {
        id: connection_id,
        name: draft.name,
        engine: draft.engine,
        host: draft.host,
        port: draft.port,
        username: draft.username,
        password,
        database: draft.database,
        tls: draft.tls,
        timeout_ms: draft.timeout_ms,
        created_at: 0,
        updated_at: 0,
    };
    engines::test_connection(&config)
}

pub fn require_connection(
    state: &State<'_, AppState>,
    id: &str,
) -> Result<ConnectionConfig, AppError> {
    let connections = state.connections.lock().unwrap();
    find_connection(&connections, id).cloned()
}
