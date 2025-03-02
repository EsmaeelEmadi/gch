use super::{ConfigEnv, ReadConfigError};
use crate::settings::Settings;

use std::fs;
use std::path::Path;

pub fn read_env(app_settings: &Settings) -> Result<ConfigEnv, ReadConfigError> {
    let current_dir = std::env::current_dir().map_err(ReadConfigError::CurrentDirError)?;

    let config_env_path = Path::new(&current_dir).join(app_settings.config_env.clone());

    let env = fs::File::open(&config_env_path).map_err(|e| ReadConfigError::FileOpenError {
        path: config_env_path.clone(),
        source: e,
    })?;

    let env_content = serde_yml::from_reader(env).map_err(ReadConfigError::DeserializeError)?;

    Ok(env_content)
}
