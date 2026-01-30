use crate::{oauth2::OAuth2Config, server::auth::AuthenticationRejection};
use aide::{OperationOutput, axum::ApiRouter, openapi::OpenApi};
use axum::{
    extract::{
        FromRef, Request,
        rejection::{JsonRejection, PathRejection, QueryRejection},
    },
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
};
use json::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use stellwerk_common::model::{id::Id, post::PostMarker, user::UserMarker};
use stellwerk_db::client::{DbClient, DbError};
use thiserror::Error;
use tracing::error;

mod auth;
mod json;
mod query;
mod routes;
mod typed_path;
mod validated;

pub type ServerRouter = ApiRouter<ServerState>;

#[derive(Clone, Debug, FromRef)]
pub struct ServerState {
    pub db_client: Arc<DbClient>,
    pub open_api: Arc<OpenApi>,
    pub oauth2_config: Arc<OAuth2Config>,
}

pub fn routes() -> ServerRouter {
    routes::routes().fallback(fallback)
}

pub async fn fallback(request: Request) -> ServerError {
    ServerError::UnknownRoute(request.into_parts().0.uri)
}

pub type Result<T, E = ServerError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Unknown route requested: {0}")]
    UnknownRoute(Uri),
    #[error("Path rejected: {0}")]
    PathRejection(#[from] PathRejection),
    #[error("Incoming JSON rejected: {0}")]
    JsonRejection(#[from] JsonRejection),
    #[error("JSON response could not be serialized: {0}")]
    JsonResponse(#[from] serde_json::Error),
    #[error("Query parameters rejected: {0}")]
    QueryRejection(#[from] QueryRejection),
    #[error(transparent)]
    AuthenticationRejection(#[from] AuthenticationRejection),
    #[error(transparent)]
    Database(#[from] DbError),
    #[error("Post with id {0} was not found.")]
    PostByIdNotFound(Id<PostMarker>),
    #[error("User with id {0} was not found.")]
    UserByIdNotFound(Id<UserMarker>),
}

// TODO: Add docs for errors (maybe once https://github.com/tamasfe/aide/pull/263 lands?)
impl OperationOutput for ServerError {
    type Inner = ServerError;
}

impl ServerError {
    pub fn status(&self) -> StatusCode {
        match self {
            ServerError::AuthenticationRejection(rejection) => rejection.status(),
            ServerError::UnknownRoute(_)
            | ServerError::PathRejection(_)
            | ServerError::PostByIdNotFound(_)
            | ServerError::UserByIdNotFound(_) => StatusCode::NOT_FOUND,
            ServerError::JsonRejection(_) | ServerError::QueryRejection(_) => {
                StatusCode::BAD_REQUEST
            }
            ServerError::JsonResponse(_) | ServerError::Database(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
struct ErrorResponse {
    status: u16,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status = self.status();

        error!(error = %self, %status, "Replying with error");

        let error_response = ErrorResponse {
            status: status.as_u16(),
        };
        (status, Json(error_response)).into_response()
    }
}
