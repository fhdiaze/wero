use super::problem::Problem;
use crate::infra::{api::problem, error::AppError};

pub fn handle(error: &AppError) -> Problem {
    let default_message = "An unexpected error occurred while processing the request";
    tracing::error!("{}", error);
    match error {
        AppError::NotFound(_) => Problem::from_kind(problem::Kind::NotFound, error.to_string()),
        AppError::Validation(_) => Problem::from_kind(problem::Kind::BadRequest, error.to_string()),
        AppError::Mongo(_) => Problem::from_kind(problem::Kind::InternalServerError, String::from(default_message)),
        AppError::Config(_) => Problem::from_kind(problem::Kind::InternalServerError, String::from(default_message)),
    }
}
