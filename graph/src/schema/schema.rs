use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use async_graphql::{EmptySubscription, Request, Response};
use async_graphql::dataloader::DataLoader;
use sqlx::PgPool;

use auth::keycloak_api::KeycloakAPI;
use auth::prelude::keycloak::KeycloakAdmin;

use crate::extension::pg_manager::PgManagerExtension;
use crate::loader::{ContentsLoader, StoryLoader, TagsLoader};
use crate::schema::{Mutation, Query};

#[derive(Clone)]
pub struct Schema(async_graphql::Schema<Query, Mutation, EmptySubscription>);

pub struct SchemaOption {
    pub db: PgPool,
    pub keycloak: KeycloakAPI,
    pub keycloak_option: KeycloakOption,
}

pub struct KeycloakOption {
    pub realm: String,
}

impl From<SchemaOption> for Schema {
    fn from(value: SchemaOption) -> Self {
        let schema = async_graphql::Schema::build(Query, Mutation, EmptySubscription)
            .data(DataLoader::new(StoryLoader(value.db.clone()), tokio::spawn))
            .data(DataLoader::new(ContentsLoader(value.db.clone()), tokio::spawn))
            .data(DataLoader::new(TagsLoader(value.db.clone()), tokio::spawn))
            .data(value.db.clone())
            .data(value.keycloak)
            .data(value.keycloak_option)
            .extension(PgManagerExtension(value.db))
            .finish();
        // schema;
        Self(schema)
    }
}

impl Deref for Schema {
    type Target = async_graphql::Schema<Query, Mutation, EmptySubscription>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Schema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}