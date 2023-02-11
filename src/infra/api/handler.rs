use super::problem::Problem;
use crate::infra::{api::problem, error::AppError};

const DEFAULT_MESSAGE: &str =
  "An unexpected error occurred while processing the request";

pub fn handle(error: &AppError) -> Problem {
  tracing::error!("{}", error);

  match error {
    AppError::NotFound(_) => {
      Problem::from_kind(problem::Kind::NotFound, error.to_string())
    }
    AppError::Validation(_) => {
      Problem::from_kind(problem::Kind::BadRequest, error.to_string())
    }
    AppError::Mongo(_) => Problem::from_kind(
      problem::Kind::InternalServerError,
      String::from(DEFAULT_MESSAGE),
    ),
    AppError::Config(_) => Problem::from_kind(
      problem::Kind::InternalServerError,
      String::from(DEFAULT_MESSAGE),
    ),
  }
}
