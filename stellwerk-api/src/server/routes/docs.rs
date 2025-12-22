use crate::server::{ServerError, ServerRouter, json::Json};
use aide::{OperationIo, openapi::OpenApi, swagger::Swagger};
use axum::extract::State;
use axum_extra::routing::TypedPath;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

pub fn routes() -> ServerRouter {
    ServerRouter::new()
        .typed_get(docs_json)
        .route("/docs", Swagger::new("/docs/json").axum_route())
}

#[derive(TypedPath, Deserialize, JsonSchema, OperationIo)]
#[aide(input)]
#[typed_path("/docs/json", rejection(ServerError))]
struct DocsJsonPath {}

async fn docs_json(
    DocsJsonPath {}: DocsJsonPath,
    State(api): State<Arc<OpenApi>>,
) -> Json<Arc<OpenApi>> {
    Json(api)
}
