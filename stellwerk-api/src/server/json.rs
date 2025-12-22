use crate::server::ServerError;
use aide::{
    OperationIo, OperationOutput,
    generate::GenContext,
    openapi,
    openapi::{Operation, StatusCode},
};
use axum::{
    Json as AxumJson,
    extract::FromRequest,
    response::{IntoResponse, Response},
};
use axum_extra::TypedHeader;
use headers::ContentType;
use serde::Serialize;

// TODO: Replace manual OperationOutput impl with generated one after https://github.com/tamasfe/aide/issues/269 is fixed
#[derive(FromRequest, OperationIo, Debug, Clone, Copy, Default)]
#[aide(input_with = "AxumJson<T>"/*, output_with = "AxumJson<T>"*/, json_schema)]
#[from_request(via(AxumJson), rejection(ServerError))]
pub struct Json<T>(pub T);

impl<T> OperationOutput for Json<T>
where
    T: schemars::JsonSchema,
{
    type Inner = T;

    fn operation_response(
        ctx: &mut GenContext,
        operation: &mut Operation,
    ) -> Option<openapi::Response> {
        AxumJson::<T>::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<StatusCode>, openapi::Response)> {
        AxumJson::<T>::inferred_responses(ctx, operation)
    }
}

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        match serde_json::to_vec(&self.0) {
            Ok(json) => (TypedHeader(ContentType::json()), json).into_response(),
            Err(err) => ServerError::JsonResponse(err).into_response(),
        }
    }
}
