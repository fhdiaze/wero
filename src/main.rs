#![deny(nonstandard_style)]
use crate::modules::race;
use axum::{http::Method, Router};
use infra::{
  config::Config,
  db::{self, traits::DynDbClient},
};
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{
  cors::{Any, CorsLayer},
  trace::TraceLayer,
};

mod domain;
mod infra;
mod modules;

fn route() -> Router<DynDbClient> {
  let races = race::controller::route();
  Router::new().nest("/api", races)
}

async fn db_client(config: &Config) -> DynDbClient {
  Arc::new(db::client::Client::new(&config.db).await.unwrap()) as DynDbClient
}

#[tokio::main]
async fn main() {
  let config = Config::new().expect("Error loading configuration");

  tracing_subscriber::fmt()
    .with_target(false)
    .compact()
    .init();

  let db_client = db_client(&config).await;
  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);
  let trace = TraceLayer::new_for_http();
  let service_builder = ServiceBuilder::new().layer(trace).layer(cors);
  let router = route().layer(service_builder).with_state(db_client);
  let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

  tracing::info!("Server is listening on address={}", addr);

  axum::Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .unwrap();
}
