use std::collections::HashMap;

use async_graphql::async_trait::async_trait;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{PgPool, query};
use uuid::Uuid;

use crate::error::LoaderError;
use crate::types::Story;

pub struct StoryLoader(pub PgPool);

#[async_trait]
impl Loader<Uuid> for StoryLoader {
    type Value = Story;
    type Error = LoaderError;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let record = query!("select story_id, title, post_at from hmb.story where story_id = any($1)", keys)
            .fetch_all(&self.0)
            .await?;
        //
        Ok(
            record.into_iter()
                  .map(|x| (x.story_id, Story {
                      story_id: x.story_id,
                      title: x.title,
                      post_at: x.post_at,
                  }))
                  .collect()
        )
    }
}