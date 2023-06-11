use async_graphql::{Context, InputType, Object, OutputType};
use async_graphql::dataloader::DataLoader;
use sqlx::query;
use uuid::Uuid;

use crate::error::LoaderError;
use crate::loader::StoryLoader;
use crate::types::{ConvertMut, MutStory};

pub struct Mutation;


#[Object]
impl Mutation {
    async fn modify_story(&self, ctx: &Context<'_>,
                          #[graphql(desc = "story id for modify")]
                          id: Uuid,
    ) -> async_graphql::Result<MutStory> {
        let data = ctx.data_unchecked::<DataLoader<StoryLoader>>().load_one(id)
                      .await?
            .ok_or_else(|| LoaderError::UnknownID {
                typename: MutStory::type_name().to_string(),
                id: id,
            })?;
        Ok(MutStory {
            source: data,
        })
    }
}