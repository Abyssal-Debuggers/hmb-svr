use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Local, NaiveDate, Utc};
use keycloak::{KeycloakAdmin, KeycloakError, KeycloakTokenSupplier};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLock;

pub type KeycloakAPI = KeycloakAdmin<KeycloakTokenManager>;

#[derive(Debug, Deserialize, Serialize)]
struct KeycloakAcquiredToken {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: usize,
    #[serde(rename = "not-before-policy")]
    not_before_policy: Option<usize>,
    refresh_expires_in: Option<usize>,
    scope: String,
    session_state: Option<String>,
    token_type: String,
}

enum TokenState {
    Ok,
    AccessTokenExpired(DateTime<Local>),
    RefreshTokenExpired(DateTime<Local>),
}


#[derive(Debug)]
pub struct KeycloakTokenManager {
    client_id: String,
    username: String,
    password: String,
    realm: String,
    url: String,
    active_token: RwLock<(KeycloakAcquiredToken, DateTime<Local>)>,
}

#[async_trait]
impl KeycloakTokenSupplier for KeycloakTokenManager {
    async fn get(&self, _url: &str) -> Result<String, KeycloakError> {
        self.refresh().await?;
        Ok(self.active_token.read().await.0.access_token.clone())
    }
}

impl KeycloakTokenManager {
    pub async fn new_admin(
        url: &str,
        username: &str,
        password: &str,
    ) -> Result<KeycloakTokenManager, KeycloakError> {
        Self::new(
            url,
            "master",
            "admin-cli",
            username,
            password,
        ).await
    }

    pub async fn new(
        url: &str,
        realm: &str,
        client_id: &str,
        username: &str,
        password: &str,
    ) -> Result<KeycloakTokenManager, KeycloakError> {
        let acquired_token = Self::acquire_password(url, realm, client_id, username, password).await?;
        Ok(KeycloakTokenManager {
            url: url.to_string(),
            active_token: RwLock::new((acquired_token, Local::now())),
            client_id: client_id.to_string(),
            realm: realm.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        })
    }
    pub async fn token_state(&self) -> TokenState {
        let active = self.active_token.read().await;
        // check access token expiration
        // -10 is the value to update the token with a margin.
        if Local::now() < active.1 + Duration::seconds(active.0.expires_in as i64 - 10) {
            return TokenState::Ok;
        }
        // check refresh token expiration
        match active.0.refresh_expires_in {
            None => {
                return TokenState::AccessTokenExpired(active.1.clone());
            }
            Some(expire_in) => {
                if Local::now() < active.1 + Duration::seconds(expire_in as i64 - 10) {
                    return TokenState::AccessTokenExpired(active.1.clone());
                }
            }
        }
        return TokenState::RefreshTokenExpired(active.1.clone());
    }

    pub async fn refresh(&self) -> Result<(), KeycloakError> {
        match self.token_state().await {
            TokenState::Ok => {
                Ok(())
            }
            TokenState::AccessTokenExpired(verified_at) => {
                let mut current_token = self.active_token.write().await;
                // protect multiple not required update
                // compare tow case
                //
                // Case 1
                // - A coroutine : token_state -> AccessTokenExpired -> !refresh -> do else
                // - B coroutine : token_state -> AccessTokenExpired -> <locked> -> !refresh -> do else
                //
                // Case 2
                // - A coroutine : token_state -> AccessTokenExpired -> !refresh -> do else
                // - B coroutine : token_state -> AccessTokenExpired -> <locked> -> <refreshed> -> do else
                //
                // case 1 refresh access token 2 times, cause after RwLock free, it do without caution
                // but case 2 do refresh only once, then if any change verified time detected
                // they think there is someone already update token
                //
                //
                // Useful if for some reason A coroutine is very delayed.
                // If A coroutine is deferred, there may be many other coroutines.
                // Think about there is no if below, all coroutines that request a refresh will update their respective access tokens.
                if verified_at != current_token.1 {
                    return Ok(());
                }
                let acquired_token = match &current_token.0.refresh_token {
                    None => Self::acquire_password(&self.url, &self.realm, &self.client_id, &self.username, &self.password).await?,
                    Some(token) => Self::acquire_refresh(&self.url, &self.realm, &self.client_id, token).await?,
                };
                current_token.0 = acquired_token;
                current_token.1 = Local::now();
                Ok(())
            }
            TokenState::RefreshTokenExpired(verified_at) => {
                let mut current_token = self.active_token.write().await;
                // see above
                if verified_at != current_token.1 {
                    return Ok(());
                }
                let acquired_token = Self::acquire_password(&self.url, &self.realm, &self.client_id, &self.username, &self.password).await?;
                current_token.0 = acquired_token;
                current_token.1 = Local::now();
                Ok(())
            }
        }
    }


    async fn acquire_password(
        url: &str,
        realm: &str,
        client_id: &str,
        username: &str,
        password: &str,
    ) -> Result<KeycloakAcquiredToken, KeycloakError> {
        let response = reqwest::Client::new()
            .post(&format!(
                "{url}/realms/{realm}/protocol/openid-connect/token",
            ))
            .form(&json!({
                "username": username,
                "password": password,
                "client_id": client_id,
                "grant_type": "password"
            }))
            .send()
            .await?;
        Ok(Self::error_check(response).await?.json::<KeycloakAcquiredToken>().await?)
    }

    async fn acquire_refresh(
        url: &str,
        realm: &str,
        client_id: &str,
        refresh_token: &str,
    ) -> Result<KeycloakAcquiredToken, KeycloakError> {
        let response = reqwest::Client::new()
            .post(&format!(
                "{url}/realms/{realm}/protocol/openid-connect/token",
            ))
            .form(&json!({
                "refresh_token": refresh_token,
                "client_id": client_id,
                "grant_type": "refresh"
            }))
            .send()
            .await?;
        Ok(Self::error_check(response).await?.json::<KeycloakAcquiredToken>().await?)
    }

    async fn error_check(response: reqwest::Response) -> Result<reqwest::Response, KeycloakError> {
        if !response.status().is_success() {
            let status = response.status().into();
            let text = response.text().await?;
            return Err(KeycloakError::HttpFailure {
                status,
                body: serde_json::from_str(&text).ok(),
                text,
            });
        }

        Ok(response)
    }
}