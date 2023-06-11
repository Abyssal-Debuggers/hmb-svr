use std::collections::HashMap;

use async_graphql::Object;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;

use auth::prelude::keycloak::types::UserRepresentation;

use crate::types::{Credential, FederatedIdentity, UserConsent};

pub struct User(UserRepresentation);

#[Object]
impl User {
    async fn access(&self) -> Option<HashMap<String, Value>> { self.0.access.clone() }
    async fn attributes(&self) -> Option<HashMap<String, Value>> { self.0.attributes.clone() }
    async fn client_consents(&self) -> Option<Vec<UserConsent>> { Some(self.0.client_consents.clone()?.into_iter().map(UserConsent::from).collect()) }
    async fn client_roles(&self) -> Option<HashMap<String, Value>> { self.0.client_roles.clone() }
    async fn created_timestamp(&self) -> Option<DateTime<Utc>> { self.0.created_timestamp.map(|x| { DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_millis(x).unwrap(), Utc) }) }
    async fn credentials(&self) -> Option<Vec<Credential>> { Some(self.0.credentials.clone()?.into_iter().map(Credential::from).collect()) }
    async fn disableable_credential_types(&self) -> Option<Vec<String>> { self.0.disableable_credential_types.clone() }
    async fn email(&self) -> Option<String> { self.0.email.clone() }
    async fn email_verified(&self) -> Option<bool> { self.0.email_verified }
    async fn enabled(&self) -> Option<bool> { self.0.enabled }
    async fn federated_identities(&self) -> Option<Vec<FederatedIdentity>> { Some(self.0.federated_identities.clone()?.into_iter().map(FederatedIdentity::from).collect()) }
    async fn federation_link(&self) -> Option<String> { self.0.federation_link.clone() }
    async fn first_name(&self) -> Option<String> { self.0.first_name.clone() }
    async fn groups(&self) -> Option<Vec<String>> { self.0.groups.clone() }
    async fn id(&self) -> Option<String> { self.0.id.clone() }
    async fn last_name(&self) -> Option<String> { self.0.last_name.clone() }
    async fn not_before(&self) -> Option<i32> { self.0.not_before }
    async fn origin(&self) -> Option<String> { self.0.origin.clone() }
    async fn realm_roles(&self) -> Option<Vec<String>> { self.0.realm_roles.clone() }
    async fn required_actions(&self) -> Option<Vec<String>> { self.0.required_actions.clone() }
    async fn _self(&self) -> Option<String> { self.0.self_.clone() }
    async fn service_account_client_id(&self) -> Option<String> { self.0.service_account_client_id.clone() }
    async fn username(&self) -> Option<String> { self.0.username.clone() }
}

impl From<UserRepresentation> for User { fn from(value: UserRepresentation) -> Self { Self(value) } }
