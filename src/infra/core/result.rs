use crate::infra::error::AppError;

pub type Result<T> = std::result::Result<T, AppError>;
