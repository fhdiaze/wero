#![deny(nonstandard_style)]
use crate::modules::race;
use axum::{
  error_handling::HandleErrorLayer,
  http::{Method, StatusCode},
  Router,
};
use infra::{
  api::handler::handle,
  config::Config,
  db::{self, traits::DynDbClient},
  error::AppError,
};
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{
  classify::{ServerErrorsAsFailures, SharedClassifier},
  cors::{Any, CorsLayer},
  trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

mod domain;
mod infra;
mod modules;

fn router() -> Router<DynDbClient> {
  let races = race::controller::route();
  Router::new().nest("/api", races)
}

fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
  TraceLayer::new_for_http()
    .make_span_with(DefaultMakeSpan::new().include_headers(true))
    .on_request(DefaultOnRequest::new().level(Level::INFO))
    .on_response(DefaultOnResponse::new().level(Level::INFO))
}

fn cors_layer() -> CorsLayer {
  CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any)
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
