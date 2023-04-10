use axum::http::StatusCode;
use serde::Serialize;

use crate::infra::error::AppError;

const DEFAULT_MESSAGE: &str =
  "An unexpected error occurred while processing the request";

#[derive(Serialize)]
pub struct Problem {
  pub status: u16,
  #[serde(rename = "type")]
  pub kind: String,
  pub title: String,
  pub detail: String,
}

#[derive(Serialize)]
pub enum Kind {
  NotFound,
  BadRequest,
  InternalServerError,
}

impl Problem {
  pub fn new(status: u16, kind: String, title: String, detail: String) -> Self {
    Problem {
      status,
      kind,
      title,
      detail,
    }
  }

  pub fn from_kind(kind: Kind, detail: String) -> Self {
    let title = kind.title();

    Self::with_title(kind, title, detail)
  }

  pub fn with_title(kind: Kind, title: String, detail: String) -> Self {
    Self::new(kind.status(), kind.kind(), title, detail)
  }

  pub fn from_error(error: &AppError) -> Problem {
    match error {
      AppError::NotFound(_) => {
        Problem::from_kind(Kind::NotFound, error.to_string())
      }
      AppError::Validation(_) => {
        Problem::from_kind(Kind::BadRequest, error.to_string())
      }
      AppError::Mongo(_) => Problem::from_kind(
        Kind::InternalServerError,
        String::from(DEFAULT_MESSAGE),
      ),
      AppError::Config(_) => Problem::from_kind(
        Kind::InternalServerError,
        String::from(DEFAULT_MESSAGE),
      ),
    }
  }
}

impl Kind {
  fn status(&self) -> u16 {
    match self {
      Kind::NotFound => StatusCode::NOT_FOUND.as_u16(),
      Kind::BadRequest => StatusCode::BAD_REQUEST.as_u16(),
      _ => StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    }
  }

  fn title(&self) -> String {
    match self {
      Kind::NotFound => {
        StatusCode::NOT_FOUND.canonical_reason().unwrap().to_owned()
      }
      Kind::BadRequest => StatusCode::BAD_REQUEST
        .canonical_reason()
        .unwrap()
        .to_owned(),
      _ => StatusCode::INTERNAL_SERVER_ERROR
        .canonical_reason()
        .unwrap()
        .to_owned(),
    }
  }

  fn kind(&self) -> String {
    match self {
      Kind::NotFound => {
        "https://tools.ietf.org/html/rfc7231#section-6.5.4".to_owned()
      }
      Kind::BadRequest => {
        "https://tools.ietf.org/html/rfc7231#section-6.5.1".to_owned()
      }
      _ => "https://tools.ietf.org/html/rfc7231#section-6.6.1".to_owned(),
    }
  }
}
