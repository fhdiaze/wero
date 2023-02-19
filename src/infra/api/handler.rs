use axum::{http::StatusCode, Json};

use super::problem::Problem;
use crate::infra::error::AppError;

pub fn handle(error: &AppError) -> (StatusCode, Json<&Problem>) {
  tracing::error!("{}", error);
  let problem = Problem::from_error(error);
  let body = Json(&problem);

  (StatusCode::from_u16(problem.status).unwrap(), body)
}
