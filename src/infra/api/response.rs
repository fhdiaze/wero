use crate::modules::race;
use axum::{
  response::{IntoResponse, Response},
  Json,
};

impl IntoResponse for race::get::RaceVm {
  fn into_response(self) -> Response {
    Json(self).into_response()
  }
}

impl IntoResponse for race::create::RaceVm {
  fn into_response(self) -> Response {
    Json(self).into_response()
  }
}
