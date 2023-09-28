mod github;
mod graphql;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;

pub fn compose() -> Router<Arc<crate::AppState>> {
    Router::new().route("/health", get(health)).nest("/github", github::routes()).nest("/graphql", graphql::routes())
}

pub(crate) async fn health(State(_state): State<Arc<crate::AppState>>) -> impl IntoResponse {
    let json: serde_json::Value = serde_json::from_str(r#"{"healthy":true}"#).unwrap();
    (StatusCode::OK, Json(json))
}
