use async_graphql::{ComplexObject, Context, SimpleObject};
use async_graphql::dataloader::DataLoader;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use entity::prelude::DateTimeWithTimeZone;
use entity::story;

use crate::loader::{ContentsLoader, TagsLoader};
use crate::types::content::Content;
use crate::types::Tag;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Story {
    pub story_id: Uuid,
    pub title: String,
    pub post_at: DateTimeWithTimeZone,
}


#[ComplexObject]
impl Story {
    async fn contents(
        &self,
        ctx: &Context<'_>,
    ) -> Vec<Content> {
        ctx.data_unchecked::<DataLoader<ContentsLoader>>()
           .load_one(self.story_id)
           .await
           .unwrap()// TODO : 에러처리가 너무 러프함
           .unwrap() // TODO : 에러처리가 너무 러프함
    }
    async fn contents_length(
        &self,
        ctx: &Context<'_>,
    ) -> usize {
        ctx.data_unchecked::<DataLoader<ContentsLoader>>()
           .load_one(self.story_id)
           .await
           .unwrap()// TODO : 에러처리가 너무 러프함
           .unwrap() // TODO : 에러처리가 너무 러프함
           .len()
    }
    async fn tags(
        &self,
        ctx: &Context<'_>,
    ) -> Vec<Tag> {
        ctx.data_unchecked::<DataLoader<TagsLoader>>()
           .load_one(self.story_id)
           .await
           .unwrap()// TODO : 에러처리가 너무 러프함
           .unwrap_or_else(|| Vec::new()) // TODO : 에러처리가 너무 러프함
    }
    async fn tags_to_string(
        &self,
        ctx: &Context<'_>,
    ) -> Vec<String> {
        ctx.data_unchecked::<DataLoader<TagsLoader>>()
           .load_one(self.story_id)
           .await
           .unwrap()// TODO : 에러처리가 너무 러프함
           .unwrap_or_else(|| Vec::new()) // TODO : 에러처리가 너무 러프함
           .into_iter()
           .map(|x| x.tag)
           .collect()
    }
}