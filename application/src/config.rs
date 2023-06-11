use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use graph::prelude::sqlx;
use graph::prelude::sqlx::{Pool, Postgres};
use graph::prelude::sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub keycloak: KeycloakConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub ip: Option<String>,
    pub port: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: Option<u32>,
    pub database: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct KeycloakConfig {
    pub url: String,
    pub realm: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        let ip = self.ip.clone().unwrap_or("0.0.0.0".to_string());
        let port = self.port.unwrap_or(80);
        return format!("{}:{}", ip, port);
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            ip: std::env::var("SERVER_IP").ok(),
            port: std::env::var("SERVER_PORT").ok().map_or(None, |x| Some(x.parse().unwrap())),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: std::env::var("DATABASE_HOST").unwrap_or("".to_string()),
            port: std::env::var("DATABASE_PORT").ok().map_or(None, |x| Some(x.parse().unwrap())),
            database: std::env::var("DATABASE_DATABASE").ok(),
            username: std::env::var("DATABASE_USERNAME").ok(),
            password: std::env::var("DATABASE_PASSWORD").ok(),
        }
    }
}

impl Default for KeycloakConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("KEYCLOAK_URL").unwrap_or("".to_string()),
            realm: std::env::var("KEYCLOAK_REALM").unwrap_or("".to_string()),
            username: std::env::var("KEYCLOAK_USERNAME").ok(),
            password: std::env::var("KEYCLOAK_PASSWORD").ok(),
        }
    }
}

impl DatabaseConfig {
    pub async fn connect(self) -> Result<Pool<Postgres>, sqlx::Error> {
        let username = self.username.unwrap_or_else(|| "postgres".to_string());
        let password = self.password.clone();
        let host = self.host.clone();
        let port = self.port.unwrap_or(5432);
        let database = self.database.unwrap_or_else(|| "postgres".to_string());

        let mut conn_opt = PgConnectOptions::new()
            .host(&host)
            .port(port as u16)
            .database(&database)
            .username(&username);
        if let Some(_pw) = password {
            conn_opt = conn_opt.password(&_pw)
        }

        let mut pool_opt = PgPoolOptions::new()
            .max_connections(4);
        pool_opt.connect_with(conn_opt).await
    }
}