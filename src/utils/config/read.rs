use super::{read_env, read_file, Config, ReadConfigError};
use crate::settings::Settings;

pub fn read(app_settings: &Settings) -> Result<Config, ReadConfigError> {
    let file = read_file(app_settings)?;
    let env = read_env(app_settings)?;

    Ok(Config { env, file })
}
