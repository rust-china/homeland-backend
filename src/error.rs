use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("code: {0}, message: `{1}`")]
    RespMessage(i32, String),
    #[error(transparent)]
    ArelError(#[from] arel::Error),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    #[error(transparent)]
    SerdeQsError(#[from] serde_qs::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ParseNumError(#[from] std::num::ParseIntError),
    #[error("`{0}`")]
    Message(String),
}

#[derive(Debug, serde::Serialize)]
struct ErrorJson {
    code: i32,
    message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let json = {
            if let Error::RespMessage(code, message) = self {
                ErrorJson { code, message }
            } else {
                ErrorJson {
                    code: 422,
                    message: format!("Error: {}", self.to_string()),
                }
            }
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json)).into_response()
    }
}
