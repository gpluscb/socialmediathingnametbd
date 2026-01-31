use oauth2::CsrfToken;
use stellwerk_common::model::{
    ModelValidationError,
    auth::Authentication,
    oauth2::{OAuth2ProviderChoice, OAuth2State, OAuth2UserIdentityDiscord},
    post::{PartialPost, Post, PostContent},
    user::{User, UserHandle},
};
use time::{Duration, PrimitiveDateTime};

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
pub(crate) struct UserRecord {
    pub user_snowflake: i64,
    pub handle: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
pub(crate) struct PostRecord {
    pub post_snowflake: i64,
    pub content: String,
    pub user_snowflake: i64,
    pub handle: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
pub(crate) struct PartialPostRecord {
    pub user_snowflake: i64,
    pub post_snowflake: i64,
    pub content: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub(crate) struct AuthenticationRecord {
    pub user_snowflake: i64,
    pub token_hash: Box<[u8]>,
    pub created_at: PrimitiveDateTime,
    pub expires_after_seconds: Option<i64>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, sqlx::Type)]
#[sqlx(type_name = "auth.oauth2_provider")]
pub(crate) enum OAuth2ProviderChoiceRecord {
    Discord,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub(crate) struct OAuth2StateRecord {
    pub session_id: String,
    pub auth_provider: OAuth2ProviderChoiceRecord,
    pub csrf_token: String,
    pub expires_at: PrimitiveDateTime,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct OAuth2UserIdentityDiscordRecord {
    pub user_snowflake: i64,
    pub oauth2_discord_id: i64,
}

impl TryFrom<UserRecord> for User {
    type Error = ModelValidationError;

    fn try_from(value: UserRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.user_snowflake.cast_unsigned().into(),
            handle: UserHandle::new(value.handle)?,
        })
    }
}

impl TryFrom<PartialPostRecord> for PartialPost {
    type Error = ModelValidationError;

    fn try_from(value: PartialPostRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.post_snowflake.cast_unsigned().into(),
            author_id: value.user_snowflake.cast_unsigned().into(),
            content: PostContent {
                content: value.content,
            },
        })
    }
}

impl TryFrom<PostRecord> for Post {
    type Error = ModelValidationError;

    fn try_from(value: PostRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.post_snowflake.cast_unsigned().into(),
            author: User {
                id: value.user_snowflake.cast_unsigned().into(),
                handle: UserHandle::new(value.handle)?,
            },
            content: PostContent {
                content: value.content,
            },
        })
    }
}

impl TryFrom<AuthenticationRecord> for Authentication {
    type Error = ModelValidationError;

    fn try_from(value: AuthenticationRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            user: value.user_snowflake.cast_unsigned().into(),
            token_hash: value.token_hash.try_into()?,
            created_at: value.created_at.as_utc(),
            expires_after: value
                .expires_after_seconds
                .map(|seconds| Duration::seconds(seconds).try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<OAuth2ProviderChoiceRecord> for OAuth2ProviderChoice {
    type Error = ModelValidationError;

    fn try_from(value: OAuth2ProviderChoiceRecord) -> Result<Self, Self::Error> {
        Ok(match value {
            OAuth2ProviderChoiceRecord::Discord => OAuth2ProviderChoice::Discord,
        })
    }
}

impl From<OAuth2ProviderChoice> for OAuth2ProviderChoiceRecord {
    fn from(value: OAuth2ProviderChoice) -> Self {
        match value {
            OAuth2ProviderChoice::Discord => OAuth2ProviderChoiceRecord::Discord,
        }
    }
}

impl TryFrom<OAuth2StateRecord> for OAuth2State {
    type Error = ModelValidationError;

    fn try_from(value: OAuth2StateRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            session_id: value.session_id,
            auth_provider: value.auth_provider.try_into()?,
            csrf_token: CsrfToken::new(value.csrf_token),
            expires_at: value.expires_at.as_utc(),
        })
    }
}

impl TryFrom<OAuth2UserIdentityDiscordRecord> for OAuth2UserIdentityDiscord {
    type Error = ModelValidationError;

    fn try_from(value: OAuth2UserIdentityDiscordRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: value.user_snowflake.cast_unsigned().into(),
            oauth2_discord_id: value.oauth2_discord_id.cast_unsigned(),
        })
    }
}
