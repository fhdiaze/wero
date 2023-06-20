use crate::infra::error::AppError;

pub type AppResult<T> = Result<T, AppError>;
