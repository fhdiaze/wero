use super::{create, find, get};
use crate::infra::{
  core::{
    paging::{Cursor, Page},
    result::AppResult,
  },
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
) -> AppResult<create::RideVm> {
  let ride = create::handle(db, cmd).await?;

  Ok(ride)
}

async fn handle_get(
  State(db): State<DynDbClient>,
  Query(query): Query<get::Query>,
) -> AppResult<get::RideVm> {
  let ride = get::handle(db, query).await?;

  Ok(ride)
}

async fn handle_find(
  State(db): State<DynDbClient>,
  Json(cursor): Json<Cursor<find::Query>>,
) -> AppResult<Page<find::RideVm>> {
  let rides = find::handle(db, cursor).await?;

  Ok(rides)
}

pub fn route() -> Router<DynDbClient> {
  Router::new()
    .route("/ride.create", post(handle_create))
    .route("/ride.get", get(handle_get))
    .route("/ride.find", post(handle_find))
}
