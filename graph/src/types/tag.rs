use std::any::{Any, TypeId};
use std::borrow::Cow;

use async_graphql::{ComplexObject, Context, ContextSelectionSet, InputType, OutputType, Positioned, ServerResult, SimpleObject, TypeName};
use async_graphql::dataloader::DataLoader;
use async_graphql::parser::types::Field;
use async_graphql::registry::Registry;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::loader::ContentsLoader;
use crate::types::content::Content;
use crate::types::ConvertMut;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Tag {
    pub name: String,
}

// async_graphql::dynamic::Enum::new()

#[ComplexObject]
impl Tag {}