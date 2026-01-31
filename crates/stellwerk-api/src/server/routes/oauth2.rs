use crate::{
    oauth2::{AuthTokenResponse, AuthUrlResponse, OAuth2Config},
    server::{
        Result, ServerError, ServerRouter, json::Json, query::Query, typed_path::PathWrapper,
    },
};
use axum::extract::State;
use axum_extra::routing::TypedPath;
use oauth2::{AuthorizationCode, CsrfToken, RedirectUrl, TokenResponse, url::Url};
use schemars::JsonSchema;
use serde::Deserialize;
use std::{borrow::Cow, sync::Arc};
use stellwerk_common::model::{auth::AuthToken, oauth2::OAuth2ProviderChoice};
use stellwerk_db::client::DbClient;

pub fn routes() -> ServerRouter {
    ServerRouter::new()
        .typed_get(get_oauth2_url)
        .typed_get(get_oauth2_authentication)
}

#[derive(TypedPath, Deserialize, JsonSchema)]
#[typed_path("/oauth/auth-url", rejection(ServerError))]
struct GetAuthUrlPath {}
#[serde_with::serde_as]
#[derive(Deserialize, JsonSchema)]
struct GetAuthUrlParams {
    provider: OAuth2ProviderChoice,
    redirect: Url,
}

async fn get_oauth2_url(
    PathWrapper(GetAuthUrlPath {}): PathWrapper<GetAuthUrlPath>,
    Query(params): Query<GetAuthUrlParams>,
    State(oauth2_config): State<Arc<OAuth2Config>>,
    State(db): State<Arc<DbClient>>,
) -> Result<Json<AuthUrlResponse>> {
    let oauth2_provider = oauth2_config.providers.get_provider(params.provider);

    let (url, csrf_token) = oauth2_provider
        .client
        .authorize_url(CsrfToken::new_random)
        .set_redirect_uri(Cow::Owned(RedirectUrl::from_url(params.redirect)))
        .add_scopes(oauth2_provider.scopes.iter().cloned())
        .url();

    todo!("Store csrf token together with auth provider choice");
    Ok(Json(AuthUrlResponse { url }))
}

#[derive(TypedPath, Deserialize, JsonSchema)]
#[typed_path("/oauth/redirect", rejection(ServerError))]
struct OauthRedirectPath {}
#[derive(Deserialize, JsonSchema)]
struct RedirectParams {
    // TODO: Maybe make this AuthorizationCode/CsrfToken directly?
    // Need to figure something out about JsonSchem
    code: String,
    state: String,
}

async fn get_oauth2_authentication(
    PathWrapper(OauthRedirectPath {}): PathWrapper<OauthRedirectPath>,
    Query(params): Query<RedirectParams>,
    State(oauth2_config): State<Arc<OAuth2Config>>,
    State(db): State<Arc<DbClient>>,
) -> Result<Json<AuthTokenResponse>> {
    let code = AuthorizationCode::new(params.code);
    let state = CsrfToken::new(params.state);

    todo!("Verify state");

    let auth_provider = &oauth2_config.providers.discord; // TODO: From state

    // Get auth provider token
    let token_response = auth_provider
        .client
        .exchange_code(code)
        .request_async(&oauth2_config.http_client)
        .await
        .expect(todo!());
    let token_type = token_response.token_type();
    let access_token = token_response.access_token();
    let refresh_token = token_response.refresh_token();
    let expires_in = token_response.expires_in();
    let scopes = token_response.scopes();

    let authorization_info = twilight_http::Client::new(access_token.into_secret())
        .current_authorization()
        .await
        .expect(todo!())
        .model()
        .await
        .expect(todo!());
    todo!("Verify identity");
    let random_token = AuthToken::generate_random(todo!("user_id"));
    let hash = random_token.hash();
    todo!("Store auth provider refresh token and generated token hash to DB");
    Ok(Json(AuthTokenResponse {
        token: random_token.token_str(),
    }))
}
