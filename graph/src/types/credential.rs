use async_graphql::Object;
use chrono::{DateTime, NaiveDateTime, Utc};

use auth::prelude::keycloak::types::CredentialRepresentation;

pub struct Credential(CredentialRepresentation);

#[Object]
impl Credential {
    async fn created_date(&self) -> Option<DateTime<Utc>> { self.0.created_date.map(|x| DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_millis(x).unwrap(), Utc)) }
    async fn credential_data(&self) -> Option<String> { self.0.credential_data.clone() }
    async fn id(&self) -> Option<String> { self.0.id.clone() }
    async fn priority(&self) -> Option<i32> { self.0.priority }
    async fn secret_data(&self) -> Option<String> { self.0.secret_data.clone() }
    async fn temporary(&self) -> Option<bool> { self.0.temporary }
    async fn _type(&self) -> Option<String> { self.0.type_.clone() }
    async fn user_label(&self) -> Option<String> { self.0.user_label.clone() }
    async fn value(&self) -> Option<String> { self.0.value.clone() }
}

impl From<CredentialRepresentation> for Credential { fn from(value: CredentialRepresentation) -> Self { Self(value) } }
