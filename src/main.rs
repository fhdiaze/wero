#![deny(nonstandard_style)]
use crate::modules::race;
use axum::Router;
use infra::{
    config,
    db::{self, traits::DynDbClient},
};
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;

mod domain;
mod infra;
mod modules;

fn route() -> Router<DynDbClient> {
    let races = race::controller::route();
    Router::new().nest("/api", races)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cfg = config::Config::new().expect("Error loading configuration");
    let db = Arc::new(db::client::Client::new(&cfg.db).await.unwrap()) as DynDbClient;
    let router = route().layer(ServiceBuilder::new()).with_state(db);
    let addr = SocketAddr::from(([0, 0, 0, 0], 7878));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    tracing::debug!("Server is listening on {}", addr);
}
