use crate::{infra::error::AppError, modules::ride};
use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};

use super::handler::handle;

impl IntoResponse for ride::get::RideVm {
  fn into_response(self) -> Response {
    Json(self).into_response()
  }
}

impl IntoResponse for ride::create::RideVm {
  fn into_response(self) -> Response {
    Json(self).into_response()
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let problem = handle(&self);
    let body = Json(&problem);

    (StatusCode::from_u16(problem.status).unwrap(), body).into_response()
  }
}
