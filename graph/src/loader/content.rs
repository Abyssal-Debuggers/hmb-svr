use std::collections::HashMap;

use async_graphql::async_trait::async_trait;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{PgPool, query};
use uuid::Uuid;

use crate::error::LoaderError;
use crate::types::Content;

pub struct ContentsLoader(pub PgPool);

#[async_trait]
impl Loader<Uuid> for ContentsLoader {
    type Value = Vec<Content>;
    type Error = LoaderError;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut result = keys.iter()
                             .map(|k| (*k, Vec::new()))
                             .collect::<HashMap<_, _>>();
        let data = query!("select * from hmb.content where story_id = any($1)", keys)
            .fetch_all(&self.0)
            .await?;
        for record in data {
            result.get_mut(&record.story_id).unwrap().push(Content {
                seq_no: record.seq_no,
                story_id: record.story_id,
                raw: record.raw,
            });
        }
        Ok(result)
    }
}