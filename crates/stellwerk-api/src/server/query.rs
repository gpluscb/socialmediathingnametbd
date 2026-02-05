#![allow(clippy::disallowed_types)]

use crate::server::ServerError;
use aide::OperationIo;
use axum::extract::{FromRequestParts, Query as AxumQuery};

#[derive(FromRequestParts, OperationIo, Debug, Clone, Copy, Default)]
#[aide(input_with = "AxumQuery<T>", json_schema)]
#[from_request(via(AxumQuery), rejection(ServerError))]
pub struct Query<T>(pub T);
