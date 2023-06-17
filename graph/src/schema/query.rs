use async_graphql::{Context, Object};
use async_graphql::connection::{Connection, Edge, EmptyFields};
use async_graphql::dataloader::DataLoader;
use sqlx::{PgPool, query};
use uuid::Uuid;

use auth::keycloak_api::KeycloakAPI;
use auth::prelude::keycloak::KeycloakAdmin;

use crate::error::LoaderError;
use crate::loader::StoryLoader;
use crate::schema::KeycloakOption;
use crate::types::{Story, User};

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
        let conn = ctx.data_unchecked::<PgPool>();
        async_graphql::connection::query(after, before, first, last, |after: Option<String>, before: Option<String>, first, last| async move {
            match (after, first, before, last) {
                (cursor, Some(size), None, None) => {
                    let start_cursor = cursor.map(|x| Uuid::try_parse(&x).unwrap());
                    let data = query!("select * from hmb.story where coalesce(story_id > $1, true) order by story_id limit $2", start_cursor, size as i64)
                        .fetch_all(conn)
                        .await?;
                    let page_info = query!(
                        r#"
                        select exists(select 1 as _i from hmb.story where story_id < $1 order by story_id limit 1) as has_prev
                             , exists(select 1 as _i from hmb.story where story_id > $2 order by story_id limit 1) as has_next
                        "#,
                        start_cursor,
                        data.last().map(|x|x.story_id)
                    )
                        .fetch_one(conn)
                        .await?;
                    let mut conn = Connection::new(
                        page_info.has_prev.unwrap_or(false),
                        page_info.has_next.unwrap_or(false),
                    );
                    conn.edges.extend(data.into_iter().map(|m|
                        Edge::with_additional_fields(
                            m.story_id.to_string(),
                            Story {
                                story_id: m.story_id,
                                title: m.title,
                                post_at: m.post_at,
                            },
                            EmptyFields,
                        )
                    ));
                    Ok(conn)
                }
                // (None, None, Some(cursor), Some(size)) => {}
                _ => Err(LoaderError::Unknown)
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


    async fn find_user_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "해당 ID의 유저정보를 가져옵니다..")]
        user_id: Uuid,
    ) -> User {
        let realm = ctx.data_unchecked::<KeycloakOption>().realm.as_str();
        ctx.data_unchecked::<KeycloakAPI>()
           .realm_users_with_id_get(realm, user_id.to_string().as_str())
           .await
           .map(|x| User::from(x))
           .unwrap()// TODO : 에러처리가 너무 러프함
    }


    async fn find_user_by_name(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "해당 ID의 유저정보를 가져옵니다..")]
        user_id: Uuid,
    ) -> User {
        let realm = ctx.data_unchecked::<KeycloakOption>().realm.as_str();
        ctx.data_unchecked::<KeycloakAPI>()
           .realm_users_with_id_get(realm, user_id.to_string().as_str())
           .await
           .map(|x| User::from(x))
           .unwrap()// TODO : 에러처리가 너무 러프함
    }
}