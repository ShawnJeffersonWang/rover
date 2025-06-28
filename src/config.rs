// src/config.rs
use serde::Deserialize;
use std::time::SystemTime;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub mode: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub max_connections: u32,
}

impl PostgresConfig {
    pub fn dsn(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.dbname
        )
    }
}

// ... 其他配置结构体 (Redis, Kafka, JWT, Snowflake) ...
#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub log: LogConfig,
    pub postgres: PostgresConfig,
    pub jwt: JwtConfig,
    // ... 其他配置 ...
}

pub fn load_config() -> anyhow::Result<Settings> {
    let builder = config::Config::builder()
        .add_source(config::File::with_name("config/default.yaml"))
        .add_source(config::Environment::with_prefix("APP").separator("__"));

    let settings = builder.build()?.try_deserialize()?;
    Ok(settings)
}