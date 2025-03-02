use super::{Config, ReadConfigError};
use crate::settings::Settings;

use std::fs;
use std::path::Path;

pub fn read(app_settings: &Settings) -> Result<Config, ReadConfigError> {
    let current_dir = std::env::current_dir().map_err(ReadConfigError::CurrentDirError)?;

    let config_file_path = Path::new(&current_dir).join(app_settings.config_file.clone());

    let file = fs::File::open(&config_file_path).map_err(|e| ReadConfigError::FileOpenError {
        path: config_file_path.clone(),
        source: e,
    })?;

    let content = serde_yml::from_reader(file).map_err(ReadConfigError::DeserializeError)?;

    Ok(content)
}
