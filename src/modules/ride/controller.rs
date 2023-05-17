use super::{create, find, get};
use crate::infra::{
  core::{
    paging::{Cursor, Page},
    result::Result,
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
) -> Result<create::RideVm> {
  let ride = create::handle(db, cmd).await?;d

  Ok(ride)
}

async fn handle_get(
  State(db): State<DynDbClient>,
  Query(query): Query<get::Query>,
) -> Result<get::RideVm> {
  let ride = get::handle(db, query).await?;x

  Ok(ride)
}

async fn handle_find(
  State(db): State<DynDbClient>,
  Json(cursor): Json<Cursor<find::Query>>,
) -> Result<Page<find::RideVm>> {
  let rides = find::handle(db, cursor).await?;

  Ok(rides)
}

pub fn route() -> Router<DynDbClient> {
  Router::new()
    .route("/ride.create", post(handle_create))
    .route("/ride.get", get(handle_get))
    .route("/ride.find", post(handle_find))
}
