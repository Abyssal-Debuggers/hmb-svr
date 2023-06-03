use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    Router,
    routing::get, Server,
};

use entity::prelude::sea_orm::Database;
use graph::prelude::async_graphql::http::GraphiQLSource;
use graph::schema::Schema;

async fn graphql_handler(
    schema: Extension<Schema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let database_option = std::env::var("DATABASE_OPTION").unwrap();
    let server_port: u32 = std::env::var("SERVER_PORT").unwrap().parse().unwrap();
    let conn = Database::connect(database_option)
        .await
        .unwrap();
    let schema = Schema::from(conn);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:{server_port}");

    let server_bind = format!("0.0.0.0:{server_port}");
    Server::bind(&server_bind.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}