use crate::error::AppError;
use crate::models::{CacheEntry, ConnectionConfig, InfoItem, KeyPage, KeySummary};
use redis::{Connection, FromRedisValue, Value};
use std::time::Duration;

fn build_url(config: &ConnectionConfig, database: Option<u8>) -> String {
    let scheme = if config.tls { "rediss" } else { "redis" };
    let username = config.username.as_deref().filter(|value| !value.is_empty());
    let password = config.password.as_deref().filter(|value| !value.is_empty());
    let auth = match (username, password) {
        (Some(user), Some(pass)) => format!(
            "{}:{}@",
            urlencoding::encode(user),
            urlencoding::encode(pass)
        ),
        (None, Some(pass)) => format!(":{}@", urlencoding::encode(pass)),
        (Some(user), None) => format!("{}@", urlencoding::encode(user)),
        (None, None) => String::new(),
    };
    let db = database.or(config.database).unwrap_or(0);
    format!("{scheme}://{auth}{}:{}/{db}", config.host.trim(), config.port)
}

fn connect(config: &ConnectionConfig, database: Option<u8>) -> Result<Connection, AppError> {
    if config.tls {
        return Err(AppError::msg(
            "当前版本暂未启用 Redis TLS，请关闭 TLS 后重试",
        ));
    }
    let client = redis::Client::open(build_url(config, database))?;
    let timeout = Duration::from_millis(config.timeout_ms.max(500));
    Ok(client.get_connection_with_timeout(timeout)?)
}

pub fn test_connection(config: &ConnectionConfig) -> Result<String, AppError> {
    let mut connection = connect(config, None)?;
    let reply: String = redis::cmd("PING").query(&mut connection)?;
    Ok(format!("Redis PING = {reply}"))
}

pub fn fetch_server_info(
    config: &ConnectionConfig,
    database: Option<u8>,
) -> Result<Vec<InfoItem>, AppError> {
    let mut connection = connect(config, database)?;
    let raw: String = redis::cmd("INFO").query(&mut connection)?;
    let wanted = [
        "redis_version",
        "os",
        "arch_bits",
        "uptime_in_seconds",
        "connected_clients",
        "used_memory_human",
        "used_memory_peak_human",
        "total_system_memory_human",
        "role",
    ];
    let mut items = Vec::new();
    for line in raw.lines() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some((key, value)) = line.split_once(':') {
            if wanted.contains(&key) || key.starts_with("db") {
                items.push(InfoItem {
                    key: key.to_string(),
                    value: value.trim().to_string(),
                });
            }
        }
    }
    Ok(items)
}

pub fn scan_keys(
    config: &ConnectionConfig,
    pattern: &str,
    cursor: Option<&str>,
    database: Option<u8>,
) -> Result<KeyPage, AppError> {
    let mut connection = connect(config, database)?;
    let match_pattern = if pattern.trim().is_empty() {
        "*"
    } else {
        pattern.trim()
    };
    let start_cursor: u64 = cursor.unwrap_or("0").parse().unwrap_or(0);
    let (next_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
        .arg(start_cursor)
        .arg("MATCH")
        .arg(match_pattern)
        .arg("COUNT")
        .arg(80)
        .query(&mut connection)?;

    Ok(KeyPage {
        keys: summarize_keys(&mut connection, &keys)?,
        cursor: Some(next_cursor.to_string()),
        done: next_cursor == 0,
        supports_scan: true,
    })
}

fn summarize_keys(connection: &mut Connection, keys: &[String]) -> Result<Vec<KeySummary>, AppError> {
    if keys.is_empty() {
        return Ok(Vec::new());
    }
    let mut pipe = redis::pipe();
    for key in keys {
        pipe.cmd("TYPE").arg(key).cmd("TTL").arg(key);
    }
    let rows: Vec<Value> = pipe.query(connection)?;
    let mut summaries = Vec::with_capacity(keys.len());
    for (index, key) in keys.iter().enumerate() {
        let type_value = rows.get(index * 2);
        let ttl_value = rows.get(index * 2 + 1);
        summaries.push(KeySummary {
            key: key.clone(),
            key_type: type_value.and_then(|value| String::from_redis_value(value).ok()),
            ttl: ttl_value.and_then(|value| i64::from_redis_value(value).ok()),
        });
    }
    Ok(summaries)
}

pub fn get_entry(
    config: &ConnectionConfig,
    key: &str,
    database: Option<u8>,
) -> Result<CacheEntry, AppError> {
    let mut connection = connect(config, database)?;
    let key_type: String = redis::cmd("TYPE").arg(key).query(&mut connection)?;
    if key_type == "none" {
        return Err(AppError::msg("键不存在"));
    }
    let ttl: i64 = redis::cmd("TTL").arg(key).query(&mut connection)?;
    let value = match key_type.as_str() {
        "string" => {
            let raw: Vec<u8> = redis::cmd("GET").arg(key).query(&mut connection)?;
            String::from_utf8(raw)
                .unwrap_or_else(|error| format!("<binary {} bytes>", error.into_bytes().len()))
        }
        "hash" => {
            let map: Vec<(String, String)> = redis::cmd("HGETALL").arg(key).query(&mut connection)?;
            serde_json::to_string_pretty(&map).unwrap_or_default()
        }
        "list" => {
            let items: Vec<String> = redis::cmd("LRANGE")
                .arg(key)
                .arg(0)
                .arg(199)
                .query(&mut connection)?;
            serde_json::to_string_pretty(&items).unwrap_or_default()
        }
        "set" => {
            let items: Vec<String> = redis::cmd("SMEMBERS").arg(key).query(&mut connection)?;
            serde_json::to_string_pretty(&items).unwrap_or_default()
        }
        "zset" => {
            let items: Vec<(String, f64)> = redis::cmd("ZRANGE")
                .arg(key)
                .arg(0)
                .arg(199)
                .arg("WITHSCORES")
                .query(&mut connection)?;
            serde_json::to_string_pretty(&items).unwrap_or_default()
        }
        other => format!("暂不支持直接编辑的类型: {other}"),
    };
    Ok(CacheEntry {
        size: value.len(),
        key: key.to_string(),
        value,
        key_type,
        ttl,
    })
}

pub fn set_entry(
    config: &ConnectionConfig,
    key: &str,
    value: &str,
    ttl_seconds: Option<u64>,
    database: Option<u8>,
) -> Result<(), AppError> {
    let mut connection = connect(config, database)?;
    match ttl_seconds.filter(|ttl| *ttl > 0) {
        Some(ttl) => {
            redis::cmd("SET")
                .arg(key)
                .arg(value)
                .arg("EX")
                .arg(ttl)
                .query::<()>(&mut connection)?;
        }
        None => {
            redis::cmd("SET")
                .arg(key)
                .arg(value)
                .query::<()>(&mut connection)?;
        }
    }
    Ok(())
}

pub fn delete_entry(
    config: &ConnectionConfig,
    key: &str,
    database: Option<u8>,
) -> Result<bool, AppError> {
    let mut connection = connect(config, database)?;
    let removed: i32 = redis::cmd("DEL").arg(key).query(&mut connection)?;
    Ok(removed > 0)
}
