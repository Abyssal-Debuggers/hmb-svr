use std::any::Any;
use std::ops::{Deref, DerefMut};

use async_graphql::{ComplexObject, Context, OutputType, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use sqlx::query;
use uuid::Uuid;

use crate::extension::pg_manager::PgManager;
use crate::guard::resource_guard::ResourceGuard;
use crate::types::{ConvertMut, ConvertUnmut};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Content {
    pub story_id: Uuid,
    pub seq_no: i32,
    #[graphql(skip)]
    pub raw: Value,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct MutContent {
    #[graphql(flatten)] pub(crate) source: Content,
    #[graphql(skip)] pub(crate) max_seq_no: i32,
}


#[ComplexObject]
impl Content {
    async fn raw(&self) -> String {
        self.raw.to_string()
    }
    // #[graphql(guard = "ResourceGuard::<Self>::new()")]
    async fn json(&self) -> Value {
        self.raw.clone()
    }
}


#[ComplexObject]
impl MutContent {
    async fn update(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "변경할 글 내용")]
        json: Option<Value>,
    ) -> async_graphql::Result<bool> {
        if json.is_none() {
            return Ok(false);
        }
        let conn = ctx.data_unchecked::<PgManager>();
        let mut tx = conn.use_tx().await?;
        if self.seq_no <= self.max_seq_no {
            query!(r#"
            with temp as (
                select story_id, seq_no from hmb.content
                where story_id = $1 and seq_no >= $2
                order by story_id desc, seq_no desc
            )
            update hmb.content c
            set seq_no = c.seq_no + 1
            from temp t
            where c.story_id = t.story_id and c.seq_no = t.seq_no
            "#, self.story_id, self.seq_no)
                .execute(tx.deref_mut())
                .await?;
        }
        let temp = query!("select * from hmb.content where story_id = $1", self.story_id)
            .fetch_all(tx.deref_mut())
            .await
            .unwrap();
        query!("insert into hmb.content(story_id, seq_no, raw) values($1, $2, $3)", self.story_id, self.seq_no, json.unwrap())
            .execute(tx.deref_mut())
            .await?;
        Ok(true)
    }


    async fn delete(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<bool> {
        let conn = ctx.data_unchecked::<PgManager>();
        let mut tx = conn.use_tx().await?;
        if self.seq_no < 1 {
            return Ok(false);
        }
        let result = query!("delete from hmb.content where story_id = $1 and seq_no = $2", self.story_id, self.seq_no)
            .execute(tx.deref_mut())
            .await?;
        Ok(result.rows_affected() > 0)
    }
}


impl Deref for MutContent {
    type Target = Content;
    fn deref(&self) -> &Self::Target { &self.source }
}

impl ConvertUnmut for MutContent {
    type Unmut = Content;
    fn into_mut(self) -> Self::Unmut { self.source }
}