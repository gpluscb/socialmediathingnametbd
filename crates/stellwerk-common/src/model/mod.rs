pub mod auth;
pub mod id;
pub mod pagination;
pub mod post;
pub mod user;

use crate::{
    model::{auth::InvalidAuthTokenHashError, user::InvalidUserHandleError},
    util::NonPositiveDurationError,
};
use thiserror::Error;
#[derive(Clone, Eq, PartialEq, Debug, Hash, Error)]
pub enum ModelValidationError {
    #[error(transparent)]
    UserHandle(#[from] InvalidUserHandleError),
    #[error(transparent)]
    NonPositiveDuration(#[from] NonPositiveDurationError),
    #[error(transparent)]
    TokenHash(#[from] InvalidAuthTokenHashError),
}
