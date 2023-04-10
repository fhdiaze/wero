use super::problem::Problem;
use crate::infra::error::AppError;

pub fn handle(error: &AppError) -> Problem {
  tracing::error!("{}", error);

  Problem::from_error(error)
}
