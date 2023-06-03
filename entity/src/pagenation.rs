use sea_orm::{DeriveColumn, EnumIter};

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum PageInfoHelper {
    Exist,
}
