use config as conf;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Validation(String),
    Mongo(mongodb::error::Error),
    Config(conf::ConfigError),
}

impl Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            AppError::NotFound(msg) => format!("NotFound: {}", msg),
            AppError::Validation(msg) => format!("Validation: {}", msg),
            AppError::Mongo(err) => format!("Mongo: {}", err),
            AppError::Config(err) => format!("Config: {}", err),
        };
        write!(f, "{}", message)
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(cause: mongodb::error::Error) -> Self {
        AppError::Mongo(cause)
    }
}

impl From<conf::ConfigError> for AppError {
    fn from(cause: conf::ConfigError) -> Self {
        AppError::Config(cause)
    }
}
