use std::fmt::{Display, Formatter};
use std::sync::Arc;

use async_graphql::{Error, Name};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum LoaderError {
    Unknown,
    DbErr(Arc<sqlx::Error>),
    UnknownID {
        typename: String,
        id: Uuid,
    },
    ValidationFailed {
        path: String,
        field: Option<String>,
        cause: String,
    },
}

impl From<sqlx::Error> for LoaderError {
    fn from(value: sqlx::Error) -> Self {
        LoaderError::DbErr(Arc::new(value))
    }
}

impl Display for LoaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(self, f) }
}