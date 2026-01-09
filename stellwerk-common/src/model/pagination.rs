use crate::model::{Id, post::PostMarker};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(tag = "pagination_reference")]
#[serde(rename_all = "snake_case")]
pub enum PaginationReference {
    Before { before: Id<PostMarker> },
    After { after: Id<PostMarker> },
    Latest,
}
