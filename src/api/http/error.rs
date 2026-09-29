use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message_error: String,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Los Datos son incorrectos")]
    DatosInvalidos,
    #[error("Error en la base de datos")]
    BaseDeDatos(#[from] sqlx::Error),
    #[error("No encontrado")]
    NoEncontrado,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self {
            ApiError::DatosInvalidos => StatusCode::BAD_REQUEST,
            ApiError::BaseDeDatos(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NoEncontrado => StatusCode::NOT_FOUND,
        };

        let respuesta = Json(ErrorResponse {
            message_error: self.to_string(),
        });
        (status, respuesta).into_response()
    }
}
