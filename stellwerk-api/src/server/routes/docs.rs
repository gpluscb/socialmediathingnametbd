use crate::server::{ServerRouter, json::Json};
use aide::{axum::routing::get, openapi::OpenApi, swagger::Swagger};
use axum::extract::State;
use std::sync::Arc;

pub fn routes() -> ServerRouter {
    ServerRouter::new()
        .route("/docs/json", get(docs_json))
        .route("/docs", Swagger::new("/docs/json").axum_route())
}

async fn docs_json(State(api): State<Arc<OpenApi>>) -> Json<Arc<OpenApi>> {
    Json(api)
}
