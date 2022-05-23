

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub app: AppSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Debug)]
pub struct AppSettings {
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_settings() -> Result<Settings, config::ConfigError> {
    config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?
        .try_deserialize()
}

pub fn get_settings_expect() -> Settings {
    get_settings().expect("bitflips::get_settings")
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        let Self { host, port, username, password, database_name, .. } = self;
        format!("postgres://{username}:{password}@{host}:{port}/{database_name}")
    }
}
