use std::fs::File;

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    Router,
    routing::get, Server,
};
use clap::Parser;

use auth::keycloak_api::{KeycloakAPI, KeycloakTokenManager};
use auth::prelude::reqwest::Client;
use graph::guard::resource_guard::{PersonalResourceData, ResourceSnippet};
use graph::prelude::async_graphql::http::GraphiQLSource;
use graph::prelude::sqlx::postgres::PgPoolOptions;
use graph::schema::{KeycloakOption, Schema, SchemaOption};

use crate::cli::CLI;
use crate::config::{Config, DatabaseConfig};

mod config;
mod cli;

async fn graphql_handler(
    schema: Extension<Schema>,
    gqlreq: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(
        gqlreq.into_inner()
              .data(PersonalResourceData::new_iter([
                  // ResourceSnippet::all_fields("Content"),
                  // ResourceSnippet::include_fields(
                  //     "Content",
                  //     ["json"],
                  // ),
              ]))
    )
          .await
          .into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let cli = CLI::parse();
    let config = File::open(cli.config)
        .map(|file| {
            let config: Config = serde_json::from_reader(file).unwrap();
            config
        })

        .unwrap_or_else(|_| Config::default());

    let conn = config.database
                     .connect()
                     .await
                     .unwrap();


    let http_client = Client::new();
    let keycloak_admin_token = KeycloakTokenManager::new_admin(
        &config.keycloak.url,
        &config.keycloak.username.expect("no keycloak username"),
        &config.keycloak.password.expect("no keycloak password"),
    )
        .await
        .unwrap();
    let keycloak_admin = KeycloakAPI::new(&config.keycloak.url, keycloak_admin_token, http_client);
    let schema = Schema::from(SchemaOption {
        db: conn,
        keycloak: keycloak_admin,
        keycloak_option: KeycloakOption {
            realm: config.keycloak.realm,
        },
    });

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:{}", config.server.port.unwrap_or(80));

    let server_bind = config.server.address();
    Server::bind(&server_bind.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}