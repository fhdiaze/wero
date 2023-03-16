use super::{create, find, get};
use crate::infra::{
  core::{page::Page, result::Result},
  db::traits::DynDbClient,
};
use axum::{
  extract::{Query, State},
  routing::{get, post},
  Json, Router,
};

async fn handle_create(
  State(db): State<DynDbClient>,
  Json(cmd): Json<create::Command>,
) -> Result<create::RideVm> {
  let race = create::handle(db, cmd).await?;

  Ok(race)
}

async fn handle_get(
  State(db): State<DynDbClient>,
  Query(query): Query<get::Query>,
) -> Result<get::RideVm> {
  let race = get::handle(db, query).await?;

  Ok(race)
}

async fn handle_find(
  State(db): State<DynDbClient>,
  Query(query): Query<find::Query>,
) -> Result<Page<find::RideVm>> {
  let query = find::Query::new(query.name, query.city, query.country);
  let rides = find::handle(db, query).await?;

  Ok(rides)
}

pub fn route() -> Router<DynDbClient> {
  Router::new()
    .route("/ride.create", post(handle_create))
    .route("/ride.get", get(handle_get))
    .route("/ride.find", get(handle_find))
}
