use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "story_tag", schema_name = "hmb")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub story_id: Uuid,
    #[sea_orm(primary_key)]
    pub tag: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Story,
    Tag,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Story => Entity::has_one(super::story::Entity).into(),
            Self::Tag => Entity::has_one(super::tag::Entity).into(),
        }
    }
}

impl Related<super::story::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Story.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_story_id<ID: Into<Uuid>>(story_id: ID) -> Select<Entity> {
        Self::find().filter(
            Column::StoryId.eq(story_id.into()),
        )
    }

    pub fn find_by_story_ids<IDS: IntoIterator<Item=impl Into<Uuid>>>(story_ids: IDS) -> Select<Entity> {
        Self::find()
            .filter(
                Column::StoryId.is_in(story_ids.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            )
    }
}