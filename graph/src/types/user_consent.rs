use async_graphql::Object;
use chrono::{DateTime, NaiveDateTime, Utc};

use auth::prelude::keycloak::types::UserConsentRepresentation;

// use sqlx::types::chrono::{NaiveDateTime, Utc};

pub struct UserConsent(UserConsentRepresentation);

#[Object]
impl UserConsent {
    async fn client_id(&self) -> Option<String> { self.0.client_id.clone() }
    async fn created_date(&self) -> Option<DateTime<Utc>> { self.0.created_date.map(|x| DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_millis(x).unwrap(), Utc)) }
    async fn granted_client_scopes(&self) -> Option<Vec<String>> { self.0.granted_client_scopes.clone() }
    async fn last_updated_date(&self) -> Option<DateTime<Utc>> { self.0.last_updated_date.map(|x| DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_millis(x).unwrap(), Utc)) }
}

impl From<UserConsentRepresentation> for UserConsent { fn from(value: UserConsentRepresentation) -> Self { Self(value) } }
