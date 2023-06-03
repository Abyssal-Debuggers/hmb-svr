use std::collections::HashMap;

use async_graphql::async_trait::async_trait;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use uuid::Uuid;

use entity::content;
use entity::prelude::{DbConn, DbErr};

use crate::loader::error::LoaderError;
use crate::types::Content;

pub struct ContentsLoader(pub DbConn);

#[async_trait]
impl Loader<Uuid> for ContentsLoader {
    type Value = Vec<Content>;
    type Error = LoaderError;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        Ok(content::Entity::find_by_story_ids(keys.to_vec())
            .all(&self.0)
            .await?
            .into_iter()
            .map(|m| (m.story_id, Content {
                seq_no: m.seq_no,
                raw: m.raw,
            }))
            .into_group_map()
        )
    }
}