use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Cfg {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub base: String,
    pub service: ServiceConfig,
    pub log_level: LogLevel,
}

#[derive(Deserialize, Clone)]
pub struct ServiceConfig {
    pub secret: String,
    pub challenge_timeout: i64,
    pub token_timeout: i64,
}

#[derive(Deserialize, Clone)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
