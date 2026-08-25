use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CacheEngine {
    Redis,
    Memcached,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredConnection {
    pub id: String,
    pub name: String,
    pub engine: CacheEngine,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub database: Option<u8>,
    #[serde(default)]
    pub tls: bool,
    pub timeout_ms: u64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub id: String,
    pub name: String,
    pub engine: CacheEngine,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<u8>,
    pub tls: bool,
    pub timeout_ms: u64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionPublic {
    pub id: String,
    pub name: String,
    pub engine: CacheEngine,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub has_password: bool,
    pub database: Option<u8>,
    pub tls: bool,
    pub timeout_ms: u64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LegacyStoredConnection {
    pub id: String,
    pub name: String,
    pub engine: CacheEngine,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    pub database: Option<u8>,
    #[serde(default)]
    pub tls: bool,
    pub timeout_ms: u64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<&ConnectionConfig> for StoredConnection {
    fn from(config: &ConnectionConfig) -> Self {
        Self {
            id: config.id.clone(),
            name: config.name.clone(),
            engine: config.engine.clone(),
            host: config.host.clone(),
            port: config.port,
            username: config.username.clone(),
            database: config.database,
            tls: config.tls,
            timeout_ms: config.timeout_ms,
            created_at: config.created_at,
            updated_at: config.updated_at,
        }
    }
}

impl From<&ConnectionConfig> for ConnectionPublic {
    fn from(config: &ConnectionConfig) -> Self {
        Self {
            id: config.id.clone(),
            name: config.name.clone(),
            engine: config.engine.clone(),
            host: config.host.clone(),
            port: config.port,
            username: config.username.clone(),
            has_password: config
                .password
                .as_ref()
                .is_some_and(|value| !value.is_empty()),
            database: config.database,
            tls: config.tls,
            timeout_ms: config.timeout_ms,
            created_at: config.created_at,
            updated_at: config.updated_at,
        }
    }
}

impl StoredConnection {
    pub fn into_config(self, password: Option<String>) -> ConnectionConfig {
        ConnectionConfig {
            id: self.id,
            name: self.name,
            engine: self.engine,
            host: self.host,
            port: self.port,
            username: self.username,
            password,
            database: self.database,
            tls: self.tls,
            timeout_ms: self.timeout_ms,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl LegacyStoredConnection {
    pub fn into_stored(self) -> StoredConnection {
        StoredConnection {
            id: self.id,
            name: self.name,
            engine: self.engine,
            host: self.host,
            port: self.port,
            username: self.username,
            database: self.database,
            tls: self.tls,
            timeout_ms: self.timeout_ms,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDraft {
    pub id: Option<String>,
    pub name: String,
    pub engine: CacheEngine,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<u8>,
    #[serde(default)]
    pub tls: bool,
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
}

fn default_timeout_ms() -> u64 {
    3000
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeySummary {
    pub key: String,
    pub key_type: Option<String>,
    pub ttl: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyPage {
    pub keys: Vec<KeySummary>,
    pub cursor: Option<String>,
    pub done: bool,
    pub supports_scan: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheEntry {
    pub key: String,
    pub value: String,
    pub key_type: String,
    pub ttl: i64,
    pub size: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoItem {
    pub key: String,
    pub value: String,
}

pub struct AppState {
    pub connections: Mutex<Vec<ConnectionConfig>>,
}

impl ConnectionDraft {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("连接名称不能为空".into());
        }
        if self.host.trim().is_empty() {
            return Err("主机地址不能为空".into());
        }
        if self.port == 0 {
            return Err("端口号无效".into());
        }
        Ok(())
    }
}
