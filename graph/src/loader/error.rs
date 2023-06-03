use std::sync::Arc;

use entity::prelude::DbErr;

#[derive(Clone, Debug)]
pub enum LoaderError {
    DbErr(Arc<DbErr>)
}

impl From<DbErr> for LoaderError {
    fn from(value: DbErr) -> Self {
        LoaderError::DbErr(Arc::new(value))
    }
}

