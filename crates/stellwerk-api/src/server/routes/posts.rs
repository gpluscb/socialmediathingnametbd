use crate::server::{
    Result, ServerError, ServerRouter, auth::AuthenticatedUser, json::Json, query::Query,
    typed_path::PathWrapper, validated::Validated,
};
use axum::extract::State;
use axum_extra::routing::TypedPath;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;
use stellwerk_common::model::{
    id::Id,
    pagination::PaginationReference,
    post::{PartialPost, Post, PostContent, PostMarker},
};
use stellwerk_db::client::DbClient;
use validator::Validate;

pub fn routes() -> ServerRouter {
    ServerRouter::new()
        .typed_get(get_post)
        .typed_get(get_recent_posts)
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
#[typed_path("/posts/recent", rejection(ServerError))]
struct RecentPostsPath {}
#[derive(Deserialize, JsonSchema, Validate)]
struct RecentPostsParams {
    #[validate(range(max = 50))]
    per_page: u32,
    #[serde(flatten)]
    // TODO: This is a hack. Without this, openapi-typescript generates wrong types.
    // Because of query parameter exploding this should be a correct OpenAPI spec anyway.
    // However, Swagger doesn't understand this so it's not ideal.
    // The hack is required, otherwise pagination_reference is not registered as a required param.
    // But could be investigated whether there's a Schemars bug or made better with a manual impl.
    #[schemars(!flatten)]
    pagination_reference: PaginationReference,
}

async fn get_recent_posts(
    PathWrapper(RecentPostsPath {}): PathWrapper<RecentPostsPath>,
    Query(params): Query<Validated<RecentPostsParams>>,
    State(db): State<Arc<DbClient>>,
) -> Result<Json<Vec<Post>>> {
    let params = params.get();
    let posts = db
        .fetch_recent_posts(params.pagination_reference, params.per_page)
        .await?;

    Ok(Json(posts))
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
