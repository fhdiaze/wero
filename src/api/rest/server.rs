use std::{net::SocketAddr, sync::Arc};

use axum::{
  http::{header::CONTENT_TYPE, Method},
  Router,
};
use tower::ServiceBuilder;
use tower_http::{
  classify::{ServerErrorsAsFailures, SharedClassifier},
  cors::{Any, CorsLayer},
  trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::{info, Level};

use crate::{
  infra::{
    config::Config,
    db::{client::Client, traits::DynDbClient},
  },
  modules::ride,
};

pub async fn start(config: &Config) {
  info!("Starting the axum server");
  let db_client = db_client(config).await;
  let service_builder = ServiceBuilder::new()
    .layer(trace_layer())
    .layer(cors_layer());
  let router = build_router().layer(service_builder).with_state(db_client);
  let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

  axum::Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .expect("Failed to load start the rest server");

  info!("Server is listening on address={}", addr);
}

pub fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
  TraceLayer::new_for_http()
    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
    .on_request(DefaultOnRequest::new().level(Level::INFO))
    .on_response(DefaultOnResponse::new().level(Level::INFO))
}

pub fn cors_layer() -> CorsLayer {
  CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([CONTENT_TYPE])
    .allow_origin(Any)
}

fn build_router() -> Router<DynDbClient> {
  let rides = ride::controller::route();

  Router::new().nest("/api", rides)
}

async fn db_client(config: &Config) -> DynDbClient {
  let client = Client::new(&config.db)
    .await
    .expect("Error creating DB client");

  Arc::new(client) as DynDbClient
}
