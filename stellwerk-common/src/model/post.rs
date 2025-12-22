use crate::model::{
    Id,
    user::{User, UserMarker},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash)]
pub struct PostMarker;

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash, Deserialize, Serialize, JsonSchema)]
pub struct Post {
    pub id: Id<PostMarker>,
    pub author: User,
    pub content: PostContent,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash, Deserialize, Serialize, JsonSchema)]
pub struct PartialPost {
    pub id: Id<PostMarker>,
    pub author_id: Id<UserMarker>,
    pub content: PostContent,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash, Deserialize, Serialize, JsonSchema)]
pub struct PostContent {
    pub content: String,
}
