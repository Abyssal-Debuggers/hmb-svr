use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql::dataloader::DataLoader;
use delegate::delegate;

use entity::prelude::{DbConn, DbErr};
use entity::prelude::sea_orm::{ConnectOptions, Database};

use crate::loader::{ContentsLoader, StoryLoader, TagsLoader};
use crate::schema::Query;

#[derive(Clone)]
pub struct Schema(async_graphql::Schema<Query, EmptyMutation, EmptySubscription>);

impl From<DbConn> for Schema {
    fn from(value: DbConn) -> Self {
        Self(
            async_graphql::Schema::build(Query, EmptyMutation, EmptySubscription)
                .data(DataLoader::new(StoryLoader(value.clone()), tokio::spawn))
                .data(DataLoader::new(ContentsLoader(value.clone()), tokio::spawn))
                .data(DataLoader::new(TagsLoader(value.clone()), tokio::spawn))
                .data(value)
                .finish()
        )
    }
}

impl Deref for Schema {
    type Target = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Schema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}