#![deny(nonstandard_style)]
use crate::modules::ride;
use axum::{http::Method, Router};
use infra::{
  config::Config,
  db::{client::Client, traits::DynDbClient},
};
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{
  classify::{ServerErrorsAsFailures, SharedClassifier},
  cors::{Any, CorsLayer},
  trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

mod domain;
mod infra;
mod modules;

fn router() -> Router<DynDbClient> {
  let rides = ride::controller::route();
  Router::new().nest("/api", rides)
}

fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
  TraceLayer::new_for_http()
    .on_request(DefaultOnRequest::new().level(Level::INFO))
    .on_response(DefaultOnResponse::new().level(Level::INFO))
}

fn cors_layer() -> CorsLayer {
  CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any)
}

async fn db_client(config: &Config) -> DynDbClient {
  let client = Client::new(&config.db)
    .await
    .expect("Error creating DB client");

  Arc::new(client) as DynDbClient
}

#[tokio::main]
async fn main() {
  let config = Config::new().expect("Error loading configuration");

  tracing_subscriber::fmt()
    .with_target(false)
    .compact()
    .init();

  let db_client = db_client(&config).await;
  let service_builder = ServiceBuilder::new()
    .layer(trace_layer())
    .layer(cors_layer());
  let router = router().layer(service_builder).with_state(db_client);
  let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

  tracing::info!("Server is listening on address={}", addr);

  axum::Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .unwrap();
}
