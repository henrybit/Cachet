use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    Message(String),
}

impl AppError {
    pub fn msg(message: impl Into<String>) -> Self {
        Self::Message(message.into())
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::Message(value.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::Message(value.to_string())
    }
}

impl From<redis::RedisError> for AppError {
    fn from(value: redis::RedisError) -> Self {
        Self::Message(format!("Redis: {value}"))
    }
}

impl From<memcache::MemcacheError> for AppError {
    fn from(value: memcache::MemcacheError) -> Self {
        Self::Message(format!("Memcached: {value}"))
    }
}
