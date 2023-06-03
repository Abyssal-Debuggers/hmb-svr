use std::collections::HashMap;

use async_graphql::async_trait::async_trait;
use async_graphql::dataloader::Loader;
use uuid::Uuid;

use entity::prelude::{DbConn, DbErr};
use entity::story;

use crate::loader::error::LoaderError;
use crate::types::Story;

pub struct StoryLoader(pub DbConn);

#[async_trait]
impl Loader<Uuid> for StoryLoader {
    type Value = Story;
    type Error = LoaderError;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        Ok(story::Entity::find_by_ids(keys.to_vec())
            .all(&self.0)
            .await?
            .into_iter()
            .map(|m| (
                m.story_id,
                Story {
                    story_id: m.story_id,
                    title: m.title,
                    post_at: m.post_at,
                }
            ))
            .collect::<HashMap<Uuid, Self::Value>>()
        )
    }
}