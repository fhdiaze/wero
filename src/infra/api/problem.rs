use axum::http::StatusCode;
use serde::Serialize;

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
        Self::new(kind.status(), kind.kind(), kind.title(), detail)
    }

    pub fn with_title(kind: Kind, title: String, detail: String) -> Self {
        Self::new(kind.status(), kind.kind(), title, detail)
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
            Kind::NotFound => StatusCode::NOT_FOUND.canonical_reason().unwrap().to_owned(),
            _ => StatusCode::INTERNAL_SERVER_ERROR
                .canonical_reason()
                .unwrap()
                .to_owned(),
        }
    }

    fn kind(&self) -> String {
        match self {
            Kind::NotFound => "https://tools.ietf.org/html/rfc7231#section-6.5.4".to_owned(),
            _ => "https://tools.ietf.org/html/rfc7231#section-6.6.1".to_owned(),
        }
    }
}
