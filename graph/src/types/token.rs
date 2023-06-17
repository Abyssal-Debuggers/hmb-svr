use std::any::{Any, TypeId};
use std::borrow::Cow;

use async_graphql::{ComplexObject, Context, ContextSelectionSet, InputType, OutputType, Positioned, ServerResult, SimpleObject, TypeName};
use async_graphql::dataloader::DataLoader;
use async_graphql::parser::types::Field;
use async_graphql::registry::Registry;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::loader::ContentsLoader;
use crate::types::content::Content;
use crate::types::ConvertMut;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, SimpleObject)]
pub struct Token {
    access: AccessToken,
    refresh: RefreshToken,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, SimpleObject)]
pub struct AccessToken {
    token: String,
    expire_at: DateTime<Utc>,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, SimpleObject)]
pub struct RefreshToken {
    token: String,
    expire_at: DateTime<Utc>,
}