use async_graphql::Object;

use auth::prelude::keycloak::types::FederatedIdentityRepresentation;

pub struct FederatedIdentity(FederatedIdentityRepresentation);

#[Object]
impl FederatedIdentity {
    async fn identity_provider(&self) -> Option<String> { self.0.identity_provider.clone() }
    async fn user_id(&self) -> Option<String> { self.0.user_id.clone() }
    async fn user_name(&self) -> Option<String> { self.0.user_name.clone() }
}


impl From<FederatedIdentityRepresentation> for FederatedIdentity { fn from(value: FederatedIdentityRepresentation) -> Self { Self(value) } }


