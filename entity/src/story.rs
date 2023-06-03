use sea_orm::{DeleteMany, QueryOrder, QuerySelect, SelectGetableValue, Selector};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Expr;
use serde::{Deserialize, Serialize};

use crate::pagenation::PageInfoHelper;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "story", schema_name = "hmb")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub story_id: Uuid,
    pub title: String,
    pub post_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Contents,
    StoryTag,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Contents => Entity::has_many(super::content::Entity).into(),
            Self::StoryTag => Entity::has_many(super::content::Entity).into(),
        }
    }
}

impl Related<super::content::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contents.def()
    }
}

impl Related<super::story_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contents.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id<ID: Into<Uuid>>(id: ID) -> Select<Entity> {
        Self::find().filter(Column::StoryId.eq(id.into()))
    }

    pub fn find_by_ids<IDS: IntoIterator<Item=impl Into<Uuid>>>(ids: IDS) -> Select<Entity> {
        Self::find()
            .filter(
                Column::StoryId.is_in(ids.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            )
    }


    pub fn page_cursor_after<ID: Into<Option<Uuid>>>(cursor: ID, size: usize) -> Select<Entity> {
        let mut query = Self::find()
            .order_by_asc(Column::StoryId)
            .limit(size as u64);
        if let Some(id) = cursor.into() {
            query = query.filter(Column::StoryId.gt(id));
        }
        query
    }
    pub fn page_cursor_after_info<ID: Into<Uuid>>(cursor: ID) -> Selector<SelectGetableValue<(i32, ), PageInfoHelper>> {
        Self::find()
            .filter(
                Column::StoryId.gt(cursor.into())
            )
            .column_as(Expr::value(1), PageInfoHelper::Exist)
            .limit(1)
            .into_values::<(i32, ), PageInfoHelper>()
    }

    pub fn page_cursor_before<ID: Into<Uuid>>(cursor: ID, size: u64) -> Select<Entity> {
        Self::find()
            .filter(
                Column::StoryId.lt(cursor.into())
            )
            .order_by_desc(Column::StoryId)
            .limit(size as u64)
    }
    pub fn page_cursor_before_info<ID: Into<Uuid>>(cursor: ID) -> Selector<SelectGetableValue<(i32, ), PageInfoHelper>> {
        Self::find()
            .filter(
                Column::StoryId.lt(cursor.into())
            )
            .column_as(Expr::value(1), PageInfoHelper::Exist)
            .limit(1)
            .into_values::<(i32, ), PageInfoHelper>()
    }


    pub fn find_by_title(title: &str) -> Select<Entity> {
        Self::find().filter(Column::Title.eq(title))
    }

    pub fn delete_by_id(id: Uuid) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::StoryId.eq(id))
    }
}