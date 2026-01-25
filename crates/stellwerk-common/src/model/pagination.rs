use crate::model::{Id, post::PostMarker};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(tag = "pagination_reference")]
#[serde(rename_all = "snake_case")]
pub enum PaginationReference {
    Newest,
    #[serde(untagged)]
    NewerThan {
        newer_than: Id<PostMarker>,
    },
    #[serde(untagged)]
    OlderThan {
        older_than: Id<PostMarker>,
    },
}
