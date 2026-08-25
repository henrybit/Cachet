use crate::error::AppError;
use crate::models::{CacheEntry, ConnectionConfig, InfoItem, KeyPage, KeySummary};
use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

fn timeout(config: &ConnectionConfig) -> Duration {
    Duration::from_millis(config.timeout_ms.max(500))
}

fn connect_url(config: &ConnectionConfig) -> String {
    let auth = match (
        config.username.as_deref().filter(|value| !value.is_empty()),
        config.password.as_deref().filter(|value| !value.is_empty()),
    ) {
        (Some(user), Some(pass)) => format!("{user}:{pass}@"),
        _ => String::new(),
    };
    format!(
        "memcache://{auth}{}:{}?timeout={}&tcp_nodelay=true",
        config.host.trim(),
        config.port,
        timeout(config).as_secs().max(1)
    )
}

fn client(config: &ConnectionConfig) -> Result<memcache::Client, AppError> {
    Ok(memcache::Client::connect(connect_url(config))?)
}

fn tcp_stream(config: &ConnectionConfig) -> Result<TcpStream, AppError> {
    let addr = (config.host.trim(), config.port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| AppError::msg("无法解析 Memcached 地址"))?;
    let stream = TcpStream::connect_timeout(&addr, timeout(config))?;
    stream.set_read_timeout(Some(timeout(config)))?;
    stream.set_write_timeout(Some(timeout(config)))?;
    Ok(stream)
}

fn ascii_command(config: &ConnectionConfig, command: &str) -> Result<String, AppError> {
    let mut stream = tcp_stream(config)?;
    stream.write_all(command.as_bytes())?;
    stream.flush()?;
    let mut buffer = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(size) => {
                buffer.extend_from_slice(&chunk[..size]);
                if buffer.windows(5).any(|window| window == b"\r\nEND")
                    || buffer.starts_with(b"ERROR")
                    || buffer.starts_with(b"CLIENT_ERROR")
                    || buffer.starts_with(b"SERVER_ERROR")
                {
                    break;
                }
            }
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                break;
            }
            Err(error) => return Err(error.into()),
        }
    }
    Ok(String::from_utf8_lossy(&buffer).to_string())
}

fn glob_match(value: &str, pattern: &str) -> bool {
    let pattern = pattern.trim();
    if pattern.is_empty() || pattern == "*" {
        return true;
    }
    let mut chars = pattern.chars().peekable();
    let mut rest = value;
    while let Some(token) = chars.next() {
        match token {
            '*' => {
                if chars.peek().is_none() {
                    return true;
                }
                let remaining: String = chars.collect();
                return rest
                    .char_indices()
                    .any(|(index, _)| glob_match(&rest[index..], &remaining));
            }
            '?' => {
                if rest.is_empty() {
                    return false;
                }
                rest = &rest[rest.chars().next().unwrap().len_utf8()..];
            }
            other => {
                if !rest.starts_with(other) {
                    return false;
                }
                rest = &rest[other.len_utf8()..];
            }
        }
    }
    rest.is_empty()
}

pub fn test_connection(config: &ConnectionConfig) -> Result<String, AppError> {
    let client = client(config)?;
    client.version()?;
    Ok("Memcached 连接成功".into())
}

pub fn fetch_server_info(config: &ConnectionConfig) -> Result<Vec<InfoItem>, AppError> {
    let client = client(config)?;
    let stats = client.stats()?;
    let mut items = Vec::new();
    let wanted = [
        "version",
        "uptime",
        "curr_items",
        "total_items",
        "bytes",
        "limit_maxbytes",
        "curr_connections",
        "evictions",
        "get_hits",
        "get_misses",
        "cmd_get",
        "cmd_set",
    ];
    for (_server, map) in stats {
        for key in wanted {
            if let Some(value) = map.get(key) {
                items.push(InfoItem {
                    key: key.to_string(),
                    value: value.clone(),
                });
            }
        }
    }
    Ok(items)
}

pub fn scan_keys(config: &ConnectionConfig, pattern: &str) -> Result<KeyPage, AppError> {
    let items_output = ascii_command(config, "stats items\r\n")?;
    if items_output.contains("CLIENT_ERROR") || items_output.contains("SERVER_ERROR") {
        return Err(AppError::msg(
            "当前 Memcached 未开放键扫描（stats cachedump）。你仍可以直接按精确键名读取。",
        ));
    }
    let mut slabs = BTreeSet::new();
    for line in items_output.lines() {
        if let Some(rest) = line.strip_prefix("STAT items:") {
            if let Some((slab, _)) = rest.split_once(':') {
                if let Ok(id) = slab.parse::<u32>() {
                    slabs.insert(id);
                }
            }
        }
    }

    let mut keys = BTreeSet::new();
    for slab in slabs {
        let dump = ascii_command(config, &format!("stats cachedump {slab} 400\r\n"))?;
        for line in dump.lines() {
            if let Some(rest) = line.strip_prefix("ITEM ") {
                if let Some(key) = rest.split_whitespace().next() {
                    if glob_match(key, pattern) {
                        keys.insert(key.to_string());
                    }
                }
            }
        }
    }

    Ok(KeyPage {
        keys: keys
            .into_iter()
            .map(|key| KeySummary {
                key,
                key_type: Some("string".into()),
                ttl: None,
            })
            .collect(),
        cursor: None,
        done: true,
        supports_scan: true,
    })
}

pub fn get_entry(config: &ConnectionConfig, key: &str) -> Result<CacheEntry, AppError> {
    let client = client(config)?;
    let value: Option<String> = client.get(key)?;
    let value = value.ok_or_else(|| AppError::msg("键不存在"))?;
    Ok(CacheEntry {
        size: value.len(),
        key: key.to_string(),
        key_type: "string".into(),
        ttl: -1,
        value,
    })
}

pub fn set_entry(
    config: &ConnectionConfig,
    key: &str,
    value: &str,
    ttl_seconds: Option<u64>,
) -> Result<(), AppError> {
    let client = client(config)?;
    let expiration = ttl_seconds.unwrap_or(0) as u32;
    client.set(key, value, expiration)?;
    Ok(())
}

pub fn delete_entry(config: &ConnectionConfig, key: &str) -> Result<bool, AppError> {
    let client = client(config)?;
    Ok(client.delete(key)?)
}
