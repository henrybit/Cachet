mod memcached_engine;
mod redis_engine;

use crate::error::AppError;
use crate::models::{CacheEngine, CacheEntry, ConnectionConfig, InfoItem, KeyPage};

pub fn test_connection(config: &ConnectionConfig) -> Result<String, AppError> {
    match config.engine {
        CacheEngine::Redis => redis_engine::test_connection(config),
        CacheEngine::Memcached => memcached_engine::test_connection(config),
    }
}

pub fn fetch_server_info(config: &ConnectionConfig, database: Option<u8>) -> Result<Vec<InfoItem>, AppError> {
    match config.engine {
        CacheEngine::Redis => redis_engine::fetch_server_info(config, database),
        CacheEngine::Memcached => memcached_engine::fetch_server_info(config),
    }
}

pub fn scan_keys(
    config: &ConnectionConfig,
    pattern: &str,
    cursor: Option<&str>,
    database: Option<u8>,
) -> Result<KeyPage, AppError> {
    match config.engine {
        CacheEngine::Redis => redis_engine::scan_keys(config, pattern, cursor, database),
        CacheEngine::Memcached => memcached_engine::scan_keys(config, pattern),
    }
}

pub fn get_entry(
    config: &ConnectionConfig,
    key: &str,
    database: Option<u8>,
) -> Result<CacheEntry, AppError> {
    match config.engine {
        CacheEngine::Redis => redis_engine::get_entry(config, key, database),
        CacheEngine::Memcached => memcached_engine::get_entry(config, key),
    }
}

pub fn set_entry(
    config: &ConnectionConfig,
    key: &str,
    value: &str,
    ttl_seconds: Option<u64>,
    database: Option<u8>,
) -> Result<(), AppError> {
    match config.engine {
        CacheEngine::Redis => redis_engine::set_entry(config, key, value, ttl_seconds, database),
        CacheEngine::Memcached => memcached_engine::set_entry(config, key, value, ttl_seconds),
    }
}

pub fn delete_entry(
    config: &ConnectionConfig,
    key: &str,
    database: Option<u8>,
) -> Result<bool, AppError> {
    match config.engine {
        CacheEngine::Redis => redis_engine::delete_entry(config, key, database),
        CacheEngine::Memcached => memcached_engine::delete_entry(config, key),
    }
}
