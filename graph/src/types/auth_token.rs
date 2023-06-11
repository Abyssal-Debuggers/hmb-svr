use async_graphql::{ComplexObject, SimpleObject};
use serde::Serialize;

#[derive(SimpleObject)]
pub struct AuthToken {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: usize,
    not_before_policy: Option<usize>,
    refresh_expires_in: Option<usize>,
    scope: String,
    session_state: Option<String>,
    token_type: String,
}