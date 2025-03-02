use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Settings {
    pub config_file: String,
    pub config_env: String,
}

impl Settings {
    pub fn new(config_file: &str, config_env: &str) -> Self {
        Self {
            config_file: config_file.to_string(),
            config_env: config_env.to_string(),
        }
    }
}

static GLOBAL_SETTINGS: OnceCell<Settings> = OnceCell::new();

pub fn init_settings(config: Settings) -> Result<(), &'static str> {
    GLOBAL_SETTINGS
        .set(config)
        .map_err(|_| "Settings already initialized")
}

pub fn settings() -> &'static Settings {
    GLOBAL_SETTINGS.get().expect("Settings not initialized")
}
