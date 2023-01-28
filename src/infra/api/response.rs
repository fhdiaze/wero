use super::handler::handle;
use crate::infra::error::AppError;
use crate::modules::race;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let problem = handle(&self);
        let body = Json(&problem);

        (StatusCode::from_u16(problem.status).unwrap(), body).into_response()
    }
}

impl IntoResponse for race::get::RaceVm {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for race::create::RaceVm {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
