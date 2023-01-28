use super::problem::Problem;
use crate::infra::{api::problem, error::AppError};

pub fn handle(error: &AppError) -> Problem {
    tracing::error!("{}", error);
    match error {
        AppError::NotFound(_) => Problem::from_kind(problem::Kind::NotFound, error.to_string()),
        AppError::Validation(_) => Problem::from_kind(problem::Kind::BadRequest, error.to_string()),
        AppError::Mongo(err) => Problem::from_kind(problem::Kind::InternalServerError, err.to_string()),
        AppError::Config(err) => Problem::from_kind(problem::Kind::InternalServerError, err.to_string()),
    }
}
