use crate::error::AppError;

const SERVICE: &str = "com.cachedgui.app";

pub fn set_password(connection_id: &str, password: &str) -> Result<(), AppError> {
    let entry = keyring::Entry::new(SERVICE, connection_id)
        .map_err(|error| AppError::msg(format!("无法访问钥匙串: {error}")))?;
    entry
        .set_password(password)
        .map_err(|error| AppError::msg(format!("无法保存密码: {error}")))?;
    Ok(())
}

pub fn get_password(connection_id: &str) -> Result<Option<String>, AppError> {
    let entry = keyring::Entry::new(SERVICE, connection_id)
        .map_err(|error| AppError::msg(format!("无法访问钥匙串: {error}")))?;
    match entry.get_password() {
        Ok(password) => Ok(Some(password)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(error) => Err(AppError::msg(format!("无法读取密码: {error}"))),
    }
}

pub fn delete_password(connection_id: &str) -> Result<(), AppError> {
    let entry = keyring::Entry::new(SERVICE, connection_id)
        .map_err(|error| AppError::msg(format!("无法访问钥匙串: {error}")))?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(error) => Err(AppError::msg(format!("无法删除密码: {error}"))),
    }
}
