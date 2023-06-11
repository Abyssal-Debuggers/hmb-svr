use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use async_graphql::{ComplexObject, Context, CustomValidator, Error, InputValueError, Result, SimpleObject};
use async_graphql::dataloader::DataLoader;
use async_graphql::dynamic::*;
use async_graphql::registry::Registry;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, query};
use uuid::Uuid;

use crate::error::LoaderError;
use crate::extension::pg_manager::PgManager;
use crate::loader::{ContentsLoader, TagsLoader};
use crate::types::{ConvertMut, ConvertUnmut, MutContent, Tag, TagState};
use crate::types::content::Content;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Story {
    pub story_id: Uuid,
    pub title: String,
    pub post_at: DateTime<Utc>,
}

#[ComplexObject]
impl Story {
    async fn contents(
        &self,
        ctx: &Context<'_>,
    ) -> Vec<Content> {
        ctx.data_unchecked::<DataLoader<ContentsLoader>>()
           .load_one(self.story_id)
           .await
           .unwrap()// TODO : 에러처리가 너무 러프함
           .unwrap() // TODO : 에러처리가 너무 러프함
    }
    async fn contents_length(
        &self,
        ctx: &Context<'_>,
    ) -> usize {
        ctx.data_unchecked::<DataLoader<ContentsLoader>>()
           .load_one(self.story_id)
           .await
           .unwrap()// TODO : 에러처리가 너무 러프함
           .unwrap() // TODO : 에러처리가 너무 러프함
           .len()
    }
    async fn tags(
        &self,
        ctx: &Context<'_>,
    ) -> Vec<Tag> {
        ctx.data_unchecked::<DataLoader<TagsLoader>>()
           .load_one(self.story_id)
           .await
           .unwrap()// TODO : 에러처리가 너무 러프함
           .unwrap_or_else(|| Vec::new()) // TODO : 에러처리가 너무 러프함
    }
    async fn tag_names(
        &self,
        ctx: &Context<'_>,
    ) -> Vec<String> {
        ctx.data_unchecked::<DataLoader<TagsLoader>>()
           .load_one(self.story_id)
           .await
           .unwrap()// TODO : 에러처리가 너무 러프함
           .unwrap_or_else(|| Vec::new()) // TODO : 에러처리가 너무 러프함
           .into_iter()
           .map(|x| x.name)
           .collect()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct MutStory {
    #[graphql(flatten)] pub(crate) source: Story,
}

#[ComplexObject]
impl MutStory {
    async fn insert_tag(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "태그 명, 존재하지 않는 태그를 입력시 자동으로 태그도 생성합니다.")]
        tag: String,
    ) -> Result<TagState> {
        let conn = ctx.data_unchecked::<PgManager>();
        let mut tx = conn.use_tx().await?;
        //
        let tag_contained = query!("select 1 as _t from hmb.story_tag where story_id = $1 and tag = $2", self.story_id, tag)
            .fetch_optional(tx.deref_mut())
            .await?
            .is_some();
        if tag_contained {
            return Ok(TagState::Owned);
        }
        //
        let mut result: TagState = TagState::Insert;
        let data = query!("select * from hmb.tag where tag = $1", tag)
            .fetch_optional(tx.deref_mut())
            .await?;
        if data.is_none() {
            query!("insert into hmb.tag(tag) values ($1)", tag)
                .execute(tx.deref_mut())
                .await?;
            result = TagState::InsertNewTag
        }
        query!("insert into hmb.story_tag(story_id, tag) values ($1, $2)", self.story_id, tag)
            .execute(tx.deref_mut())
            .await?;
        Ok(result)
    }
    async fn delete_tag(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "태그 명, 존재하지 않는 태그를 입력시에도 에러 없이 동작합니다.")]
        tag: String,
    ) -> Result<TagState> {
        let conn = ctx.data_unchecked::<PgManager>();
        let mut tx = conn.use_tx().await?;
        let result = query!("delete from hmb.story_tag where story_id = $1 and tag = $2", self.story_id, tag)
            .execute(tx.deref_mut())
            .await?;
        if result.rows_affected() == 0 {
            return Ok(TagState::NotOwned);
        }
        Ok(TagState::Delete)
    }
    async fn update(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "변경할 글 타이틀")]
        title: Option<String>,
        #[graphql(desc = "글 수정일 변경")]
        post_at: Option<DateTime<Utc>>,
    ) -> Result<bool> {
        if title.is_none() && post_at.is_none() {
            return Ok(false);
        }
        let conn = ctx.data_unchecked::<PgManager>();
        let mut tx = conn.use_tx().await?;
        let result = query!("update hmb.story set title = coalesce($1, title), post_at = coalesce($2, post_at) where story_id = $3", title, post_at, self.story_id)
            .execute(tx.deref_mut())
            .await?;
        Ok(true)
    }
    async fn modify_content(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "수정 위치 선택", validator(minimum = 1))]
        seq_no: Option<i32>,
    ) -> Result<MutContent> {
        let conn = ctx.data_unchecked::<PgManager>();
        let mut tx = conn.use_tx().await?;
        //
        let max_seq_no = query!("select max(seq_no) as max from hmb.content where story_id = $1", self.story_id)
            .fetch_one(tx.deref_mut())
            .await?;
        match (max_seq_no.max, seq_no) {
            (None, None) => {
                Ok(MutContent {
                    source: Content {
                        story_id: self.story_id,
                        seq_no: 1,
                        raw: Value::Null,
                    },
                    max_seq_no: 0,
                })
            }
            (None, Some(i1)) => {
                if i1 != 1 {
                    return Err(Error::new("no story content, only 1 allowed"));
                }
                Ok(MutContent {
                    source: Content {
                        story_id: self.story_id,
                        seq_no: 1,
                        raw: Value::Null,
                    },
                    max_seq_no: 0,
                })
            }
            (Some(i0), None) => {
                Ok(MutContent {
                    source: Content {
                        story_id: self.story_id,
                        seq_no: i0 + 1,
                        raw: Value::Null,
                    },
                    max_seq_no: i0,
                })
            }
            (Some(i0), Some(i1)) if i0 + 1 < i1 => {
                Err(Error::new(format!("max seq_no is {i0} so seq_no must lesser equal {})", i0 + 1)))
            }
            (Some(i0), Some(i1)) => {
                Ok(MutContent {
                    source: Content {
                        story_id: self.story_id,
                        seq_no: i1,
                        raw: Value::Null,
                    },
                    max_seq_no: i0,
                })
            }
        }
    }
}

impl Deref for MutStory {
    type Target = Story;
    fn deref(&self) -> &Self::Target { &self.source }
}


impl ConvertUnmut for MutStory {
    type Unmut = Story;
    fn into_mut(self) -> Self::Unmut { self.source }
}