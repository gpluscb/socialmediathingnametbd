use crate::server::ServerError;
use aide::{OperationInput, generate::GenContext, openapi::Operation};
use axum::{
    extract::FromRequestParts,
    http::{Uri, request::Parts},
};
use axum_extra::routing::TypedPath;
use schemars::JsonSchema;
use std::fmt::{Display, Formatter};

/// Like [`aide::axum::routing::typed::TypedPath`], but implementing required traits.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, JsonSchema)]
#[schemars(transparent)]
pub struct PathWrapper<T>(pub T);

impl<T> OperationInput for PathWrapper<T>
where
    T: axum_extra::routing::TypedPath + JsonSchema,
{
    fn operation_input(ctx: &mut GenContext, operation: &mut Operation) {
        aide::axum::routing::typed::TypedPath::<T>::operation_input(ctx, operation);
    }
}

impl<T, S> FromRequestParts<S> for PathWrapper<T>
where
    T: FromRequestParts<S>,
    ServerError: From<<T as FromRequestParts<S>>::Rejection>,
    S: Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let inner = T::from_request_parts(parts, state).await?;
        Ok(Self(inner))
    }
}

impl<T> Display for PathWrapper<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> TypedPath for PathWrapper<T>
where
    T: TypedPath,
{
    const PATH: &'static str = T::PATH;

    fn to_uri(&self) -> Uri {
        self.0.to_uri()
    }
}
