use crate::server::{
    Result, ServerError, ServerRouter, auth::AuthenticatedUser, json::Json, typed_path::PathWrapper,
};
use axum::extract::State;
use axum_extra::routing::TypedPath;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;
use stellwerk_common::model::{
    Id,
    post::{PartialPost, Post, PostContent, PostMarker},
};
use stellwerk_db::client::DbClient;

pub fn routes() -> ServerRouter {
    ServerRouter::new()
        .typed_get(get_post)
        .typed_post(create_post)
}

#[derive(TypedPath, Deserialize, JsonSchema)]
#[typed_path("/posts/{id}", rejection(ServerError))]
struct GetPostPath {
    id: Id<PostMarker>,
}

async fn get_post(
    PathWrapper(GetPostPath { id }): PathWrapper<GetPostPath>,
    State(db): State<Arc<DbClient>>,
) -> Result<Json<Post>> {
    let post = db
        .fetch_post(id)
        .await?
        .ok_or(ServerError::PostByIdNotFound(id))?;

    Ok(Json(post))
}

#[derive(TypedPath, Deserialize, JsonSchema)]
#[typed_path("/posts/create", rejection(ServerError))]
struct CreatePostPath {}

async fn create_post(
    PathWrapper(CreatePostPath {}): PathWrapper<CreatePostPath>,
    State(db): State<Arc<DbClient>>,
    user: AuthenticatedUser,
    Json(post): Json<PostContent>,
) -> Result<Json<PartialPost>> {
    let post = db.create_post(&post, user.user_id()).await?;

    Ok(Json(post))
}
