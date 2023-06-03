use std::collections::HashMap;

use async_graphql::async_trait::async_trait;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use uuid::Uuid;

use entity::prelude::{DbConn, DbErr};
use entity::story_tag;

use crate::loader::error::LoaderError;
use crate::types::Tag;

pub struct TagsLoader(pub DbConn);

#[async_trait]
impl Loader<Uuid> for TagsLoader {
    type Value = Vec<Tag>;
    type Error = LoaderError;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        Ok(story_tag::Entity::find_by_story_ids(keys.to_vec())
            .all(&self.0)
            .await?
            .into_iter()
            .map(|m| (m.story_id, Tag {
                tag: m.tag
            }))
            .into_group_map()
        )
    }
}