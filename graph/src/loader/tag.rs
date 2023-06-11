use std::collections::HashMap;

use async_graphql::async_trait::async_trait;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{PgPool, query};
use uuid::Uuid;

use crate::error::LoaderError;
use crate::types::Tag;

pub struct TagsLoader(pub PgPool);

#[async_trait]
impl Loader<Uuid> for TagsLoader {
    type Value = Vec<Tag>;
    type Error = LoaderError;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut result = keys.iter()
                             .map(|k| (*k, Vec::new()))
                             .collect::<HashMap<_, _>>();
        let data = query!("select * from hmb.story_tag where story_id = any($1)", keys)
            .fetch_all(&self.0)
            .await?;
        for record in data {
            result.get_mut(&record.story_id).unwrap().push(Tag {
                name: record.tag,
            });
        }
        Ok(result)
    }
}