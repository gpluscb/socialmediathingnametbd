use crate::config::ApiOAuth2ProvidersConfig;
use oauth2::{
    AccessToken, EndpointNotSet, EndpointSet, Scope, basic::BasicClient, reqwest,
    reqwest::redirect::Policy, url::Url,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use stellwerk_common::{
    json_schema_wrappers::JsonSchemaOffsetDateTime,
    model::{id::Id, oauth2::OAuth2ProviderChoice, user::UserMarker},
};
use stellwerk_db::client::{DbClient, DbError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OAuth2SetupError {
    #[error(transparent)]
    ReqwestConfig(#[from] reqwest::Error),
}

pub type ProviderClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointSet, EndpointSet>;

#[derive(Clone, Debug)]
pub struct OAuth2Service {
    pub providers: OAuth2ProviderList,
    pub http_client: reqwest::Client,
}

impl OAuth2Service {
    pub fn new(config: ApiOAuth2ProvidersConfig) -> Result<Self, OAuth2SetupError> {
        let config = OAuth2Service {
            providers: OAuth2ProviderList {
                discord: OAuth2Provider {
                    client: BasicClient::new(config.discord.client_id)
                        .set_client_secret(config.discord.client_secret)
                        .set_auth_uri(config.discord.auth_url)
                        .set_token_uri(config.discord.token_url)
                        .set_revocation_url(config.discord.revocation_url),
                    scopes: config.discord.scopes,
                },
            },
            http_client: reqwest::Client::builder()
                .redirect(Policy::none())
                .build()?,
        };
        Ok(config)
    }
}

#[derive(Clone, Debug)]
pub struct OAuth2Provider {
    pub client: ProviderClient,
    pub scopes: Vec<Scope>,
}

#[derive(Clone, Debug)]
pub struct OAuth2ProviderList {
    pub discord: OAuth2Provider,
}

impl OAuth2ProviderList {
    #[must_use]
    pub fn get_provider(&self, provider_choice: OAuth2ProviderChoice) -> &OAuth2Provider {
        match provider_choice {
            OAuth2ProviderChoice::Discord => &self.discord,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize, JsonSchema)]
pub struct AuthUrlResponse {
    pub url: Url,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenResponse {
    pub token: String,
    // TODO: Replace with UtcDateTime if https://github.com/GREsau/schemars/pull/472/ ever lands
    pub expires_at: Option<JsonSchemaOffsetDateTime>,
}

#[derive(Debug, Error)]
pub enum OAuth2IdentityRetrievalError {
    #[error(transparent)]
    DiscordRequest(#[from] twilight_http::Error),
    #[error(transparent)]
    DiscordResponseDeserialize(#[from] twilight_http::response::DeserializeBodyError),
    #[error("Discord identify response did not contain user")]
    DiscordResponseUserNotPresent,
    #[error(transparent)]
    Database(#[from] DbError),
}

pub async fn get_identity_from_provider(
    db: &DbClient,
    oauth2provider_choice: OAuth2ProviderChoice,
    access_token: AccessToken,
) -> Result<Option<Id<UserMarker>>, OAuth2IdentityRetrievalError> {
    match oauth2provider_choice {
        OAuth2ProviderChoice::Discord => get_identity_from_discord(db, access_token).await,
    }
}

pub async fn get_identity_from_discord(
    db: &DbClient,
    access_token: AccessToken,
) -> Result<Option<Id<UserMarker>>, OAuth2IdentityRetrievalError> {
    let authorization_info =
        twilight_http::Client::new(format!("Bearer {}", access_token.into_secret()))
            .current_authorization()
            .await?
            .model()
            .await?;

    let discord_id = authorization_info
        .user
        .ok_or(OAuth2IdentityRetrievalError::DiscordResponseUserNotPresent)?
        .id;

    let identity = db.fetch_oauth2_identity_discord(discord_id.get()).await?;

    let user_id = identity.map(|identity| identity.user_id);
    Ok(user_id)
}
