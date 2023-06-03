use async_graphql::{ComplexObject, Context, SimpleObject};
use async_graphql::dataloader::DataLoader;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use entity::prelude::DateTimeWithTimeZone;
use entity::story;

use crate::loader::ContentsLoader;
use crate::types::content::Content;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Tag {
    pub tag: String,
}


#[ComplexObject]
impl Tag {}