use oauth2::{
    AccessToken, AuthUrl, EndpointNotSet, EndpointSet, RevocationUrl, Scope, TokenUrl,
    basic::BasicClient, reqwest, reqwest::redirect::Policy, url::Url,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use stellwerk_common::model::{id::Id, oauth2::OAuth2ProviderChoice, user::UserMarker};
use stellwerk_db::client::{DbClient, DbError};
use thiserror::Error;

// TODO: Maybe a way to deserialize from toml?
#[must_use]
pub fn get_oauth2_config() -> OAuth2Config {
    OAuth2Config {
        providers: OAuth2ProviderList {
            discord: OAuth2Provider {
                client: BasicClient::new(todo!("client_id"))
                    .set_client_secret(todo!("client_secret"))
                    .set_auth_uri(
                        AuthUrl::new("https://discord.com/oauth2/authorize".to_string())
                            .expect(todo!()),
                    )
                    .set_token_uri(
                        TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).unwrap(),
                    )
                    .set_revocation_url(
                        RevocationUrl::new(
                            "https://discord.com/api/oauth2/token/revoke".to_string(),
                        )
                        .expect(todo!()),
                    ),
                scopes: vec![Scope::new("identify".to_string())],
            },
        },
        http_client: reqwest::Client::builder()
            .redirect(Policy::none())
            .build()
            .expect(todo!()),
    }
}

pub type ProviderClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointSet, EndpointSet>;

#[derive(Clone, Debug)]
pub struct OAuth2Config {
    pub providers: OAuth2ProviderList,
    pub http_client: reqwest::Client,
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

#[serde_with::serde_as]
#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize, JsonSchema)]
pub struct AuthUrlResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub url: Url,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenResponse {
    // TODO: Maybe make this AuthToken?
    pub token: String,
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
    let authorization_info = twilight_http::Client::new(access_token.into_secret())
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
