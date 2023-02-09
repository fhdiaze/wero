use crate::infra::core::result::Result;
use config as conf;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum Env {
    Dev,
    Test,
    Prod,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub env: Env,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Db {
    pub connection_string: String,
    pub db_name: String,
    pub races_collection: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub log: Log,
    pub db: Db,
}

impl Config {
    pub fn new() -> Result<Self> {
        let builder = conf::Config::builder()
            .add_source(conf::File::with_name("./config/default.toml"))
            .add_source(
                conf::Environment::with_prefix("WERO")
                    .prefix_separator("_")
                    .separator("__")
                    .ignore_empty(true),
            )
            .add_source(
                conf::Environment::with_prefix("CUSTOMCONNSTR")
                    .prefix_separator("_")
                    .separator("__")
                    .ignore_empty(true),
            );

        let config: Config = builder.build()?.try_deserialize()?;

        Ok(config)
    }
}
