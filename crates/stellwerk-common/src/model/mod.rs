pub mod auth;
pub mod id;
pub mod oauth2;
pub mod pagination;
pub mod post;
pub mod user;

use crate::{
    model::{auth::InvalidAuthTokenHashError, user::InvalidUserHandleError},
    positive_duration::NonPositiveDurationError,
};
use ::oauth2::url;
use thiserror::Error;

#[derive(Clone, Eq, PartialEq, Debug, Error)]
pub enum ModelValidationError {
    #[error(transparent)]
    UserHandle(#[from] InvalidUserHandleError),
    #[error(transparent)]
    NonPositiveDuration(#[from] NonPositiveDurationError),
    #[error(transparent)]
    TokenHash(#[from] InvalidAuthTokenHashError),
    #[error("Redirect url is invalid: {0}")]
    InvalidRedirectUrl(#[from] url::ParseError),
}
