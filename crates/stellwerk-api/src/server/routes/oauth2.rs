use crate::{
    oauth2::{AuthTokenResponse, AuthUrlResponse, OAuth2Config, get_identity_from_provider},
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
use stellwerk_common::{
    model::{
        auth::{AuthToken, Authentication},
        oauth2::{OAuth2ProviderChoice, OAuth2State},
    },
    util::PositiveDuration,
};
use stellwerk_db::client::DbClient;
use time::{Duration, UtcDateTime};

pub fn routes() -> ServerRouter {
    ServerRouter::new()
        .typed_get(get_oauth2_url)
        .typed_get(get_token)
}

#[derive(TypedPath, Deserialize, JsonSchema)]
#[typed_path("/oauth2/auth-url", rejection(ServerError))]
struct GetAuthUrlPath {}
#[derive(Deserialize, JsonSchema)]
struct GetAuthUrlParams {
    provider: OAuth2ProviderChoice,
    redirect: Url,
    session_id: String,
}

async fn get_oauth2_url(
    PathWrapper(GetAuthUrlPath {}): PathWrapper<GetAuthUrlPath>,
    Query(params): Query<GetAuthUrlParams>,
    State(oauth2_config): State<Arc<OAuth2Config>>,
    State(db): State<Arc<DbClient>>,
) -> Result<Json<AuthUrlResponse>> {
    let oauth2_provider = oauth2_config.providers.get_provider(params.provider);
    let redirect_url = RedirectUrl::from_url(params.redirect);

    let (url, csrf_token) = oauth2_provider
        .client
        .authorize_url(CsrfToken::new_random)
        .set_redirect_uri(Cow::Borrowed(&redirect_url))
        .add_scopes(oauth2_provider.scopes.iter().cloned())
        .url();

    let oauth2_state = OAuth2State {
        session_id: params.session_id,
        auth_provider: params.provider,
        csrf_token,
        redirect_url,
        expires_at: UtcDateTime::now() + Duration::minutes(30),
    };
    db.create_oauth2_state(&oauth2_state).await?;

    Ok(Json(AuthUrlResponse { url }))
}

#[derive(TypedPath, Deserialize, JsonSchema)]
#[typed_path("/oauth2/get-token", rejection(ServerError))]
struct GetTokenPath {}
#[derive(Deserialize, JsonSchema)]
struct GetTokenParams {
    code: String,
    csrf_token: String,
    session_id: String,
    expires: bool,
}

async fn get_token(
    PathWrapper(GetTokenPath {}): PathWrapper<GetTokenPath>,
    Query(params): Query<GetTokenParams>,
    State(oauth2_config): State<Arc<OAuth2Config>>,
    State(db): State<Arc<DbClient>>,
) -> Result<Json<AuthTokenResponse>> {
    let code = AuthorizationCode::new(params.code);
    let csrf_token = CsrfToken::new(params.csrf_token);

    let stored_oauth2_state = db
        .fetch_oauth2_state(&params.session_id)
        .await?
        .ok_or_else(|| ServerError::OAuth2NoStateForSession(params.session_id.clone()))?;

    if stored_oauth2_state.expires_at < UtcDateTime::now() {
        return Err(ServerError::OAuth2NoStateForSession(params.session_id));
    }

    if stored_oauth2_state.csrf_token != csrf_token {
        return Err(ServerError::OAuth2WrongCsrfToken);
    }

    // State has been used and can be deleted
    // Since we don't use transactions, a user could use a race to use the same oauth2 state
    // multiple times to generate multiple tokens, but this is not harmful.
    db.delete_oauth2_state(&params.session_id).await?;

    let auth_provider = oauth2_config
        .providers
        .get_provider(stored_oauth2_state.auth_provider);

    // Get auth provider token
    let token_response = auth_provider
        .client
        .exchange_code(code)
        .set_redirect_uri(Cow::Owned(stored_oauth2_state.redirect_url))
        .request_async(&oauth2_config.http_client)
        .await?;
    let access_token = token_response.access_token();

    // TODO: Maybe we should revoke if this fails also
    // Use auth provider token to verify identity
    let user_id =
        get_identity_from_provider(&db, stored_oauth2_state.auth_provider, access_token.clone())
            .await?
            .ok_or(ServerError::OAuth2NoAssociatedUser)?;

    // Auth provider token is useless after identity verification, revoke
    auth_provider
        .client
        .revoke_token(access_token.into())?
        .request_async(&oauth2_config.http_client)
        .await?;

    // Generate new api token for user
    let random_token = AuthToken::generate_random(user_id);
    let hash = random_token.hash()?;

    let expires_after = if params.expires {
        // TODO: This duration should probably be configurable for the server
        Some(PositiveDuration::new_unchecked(Duration::days(1)))
    } else {
        None
    };

    let authentication = Authentication {
        user: user_id,
        token_hash: hash,
        created_at: UtcDateTime::now(),
        expires_after,
    };

    // Store newly created authentication
    db.create_auth(&authentication).await?;

    Ok(Json(AuthTokenResponse {
        token: random_token.token_str(),
    }))
}
