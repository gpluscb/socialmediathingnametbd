use crate::model::{id::Id, user::UserMarker};
use oauth2::CsrfToken;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use time::UtcDateTime;

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize, JsonSchema,
)]
pub enum OAuth2ProviderChoice {
    Discord,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct OAuth2State {
    pub session_id: String,
    pub auth_provider: OAuth2ProviderChoice,
    pub csrf_token: CsrfToken,
    pub expires_at: UtcDateTime,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct OAuth2UserIdentityDiscord {
    pub user_id: Id<UserMarker>,
    pub oauth2_discord_id: u64,
}
