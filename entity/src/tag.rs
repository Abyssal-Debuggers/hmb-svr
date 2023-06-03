use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tag", schema_name = "hmb")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub tag: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    StoryTag,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::StoryTag => Entity::has_many(super::story_tag::Entity).into(),
        }
    }
}

impl Related<super::story_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StoryTag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {}