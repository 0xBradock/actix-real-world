use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::new("config.yaml", FileFormat::Yaml))
            .build()?;

        s.try_deserialize()
    }

    pub fn db_connection_string(&self, db_name: &str) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.database.username, self.database.password, self.database.host, db_name
        )
    }
}
