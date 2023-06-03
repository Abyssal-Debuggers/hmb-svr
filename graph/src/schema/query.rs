use async_graphql::{Context, Object};
use async_graphql::connection::{Connection, Edge, EmptyFields, query};
use async_graphql::dataloader::DataLoader;
use uuid::Uuid;

use entity::prelude::DbConn;
use entity::prelude::sea_orm::QuerySelect;
use entity::prelude::sea_orm::sea_query::Expr;
use entity::story;

use crate::loader::StoryLoader;
use crate::types::Story;

pub struct Query;


#[Object]
impl Query {
    async fn find_story(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "페이징 관련")]
        after: Option<String>,
        #[graphql(desc = "페이징 관련")]
        before: Option<String>,
        #[graphql(desc = "페이징 관련")]
        first: Option<i32>,
        #[graphql(desc = "페이징 관련")]
        last: Option<i32>,
    ) -> Connection<String, Story, EmptyFields, EmptyFields> {
        let dbconn = ctx.data_unchecked::<DbConn>();
        query(after, before, first, last,
              |after: Option<String>, before: Option<String>, first, last| async move {
                  match (after, first, before, last) {
                      (cursor, Some(size), None, None) => {
                          let start_cursor = cursor.map(|x| Uuid::try_parse(&x).unwrap());
                          let data = story::Entity::page_cursor_after(start_cursor.clone(), size)
                              .all(dbconn)
                              .await
                              .unwrap();

                          let has_next = match data.last() {
                              Some(data) => story::Entity::page_cursor_after_info(data.story_id)
                                  .one(dbconn)
                                  .await
                                  .unwrap()
                                  .is_some(),
                              None => false,
                          };
                          //
                          let has_prev = match start_cursor {
                              Some(data) => story::Entity::page_cursor_before_info(data)
                                  .one(dbconn)
                                  .await
                                  .unwrap()
                                  .is_some(),
                              None => false,
                          };

                          let mut conn = Connection::new(has_prev, has_next);
                          conn.edges.extend(data.into_iter().map(|m| Edge::with_additional_fields(m.story_id.to_string(), Story {
                              story_id: m.story_id,
                              title: m.title,
                              post_at: m.post_at,
                          }, EmptyFields)));
                          Ok(conn)
                      }
                      // (None, None, Some(cursor), Some(size)) => {}
                      _ => Err("")
                  }
              },
        ).await
         .unwrap()
    }
    async fn find_story_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "단 하나의 이야기를 아이디로 가져옵니다.")]
        story_id: Uuid,
    ) -> Story {
        ctx.data_unchecked::<DataLoader<StoryLoader>>().load_one(story_id)
           .await
           .unwrap()// TODO : 에러처리가 너무 러프함
           .unwrap() // TODO : 에러처리가 너무 러프함
    }
}