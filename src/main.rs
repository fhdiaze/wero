#![deny(nonstandard_style)]
use std::str::FromStr;

use api::rest;
use infra::config::Config;
use tracing::Level;
use tracing_subscriber::EnvFilter;

mod api;
mod domain;
mod infra;
mod modules;

#[tokio::main]
async fn main() {
  let config = Config::new().expect("Failed to load configuration");
  add_logger(&config);
  rest::server::start(&config).await;
}

fn add_logger(config: &Config) {
  let filter_layer = EnvFilter::default()
    .add_directive(Level::from_str(&config.log.level).unwrap().into());
  tracing_subscriber::fmt()
    .with_env_filter(filter_layer)
    .with_target(false)
    .with_file(true)
    .json()
    .init();
}
