use async_graphql::Enum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Serialize, Deserialize, Enum)]
pub enum TagState {
    Owned,
    Insert,
    InsertNewTag,
    Delete,
    NotOwned,
}