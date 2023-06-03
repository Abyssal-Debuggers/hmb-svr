use sea_orm::DeleteMany;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Cond;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "content", schema_name = "hmb")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub story_id: Uuid,
    #[sea_orm(primary_key)]
    pub seq_no: i32,
    pub raw: Json,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Story
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Story => Entity::has_one(super::story::Entity).into(),
        }
    }
}

impl Related<super::story::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Story.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(story_id: Uuid, seq_no: i32) -> Select<Entity> {
        Self::find()
            .filter(
                Cond::all()
                    .add(Column::StoryId.eq(story_id))
                    .add(Column::SeqNo.eq(seq_no))
            )
    }

    pub fn find_by_story_id(story_id: Uuid) -> Select<Entity> {
        Self::find()
            .filter(
                Column::StoryId.eq(story_id)
            )
    }

    pub fn find_by_story_ids<IDS: IntoIterator<Item=impl Into<Uuid>>>(story_ids: IDS) -> Select<Entity> {
        Self::find()
            .filter(
                Column::StoryId.is_in(story_ids.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            )
    }


    pub fn find_by_ids<IDS: IntoIterator<Item=(Uuid, i32)>>(id: IDS) -> Select<Entity> {
        Self::find()
            .filter(
                id.into_iter()
                  .map(|(x, y)|
                      Cond::all()
                          .add(Column::StoryId.eq(x))
                          .add(Column::SeqNo.eq(y))
                  )
                  .fold(Cond::any(), |acc, elem| acc.add(elem))
            )
    }


    pub fn delete_by_id(story_id: Uuid, seq_no: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(
            Cond::all()
                .add(Column::StoryId.eq(story_id))
                .add(Column::SeqNo.eq(seq_no))
        )
    }
}