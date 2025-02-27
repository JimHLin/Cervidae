use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
};
use dotenvy;
use sqlx::PgPool;
use std::env;
use storage::{MutationRoot, QueryRoot};
use tokio::net::TcpListener;

pub mod models;
mod storage;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    // setup database connection
    let pool = PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to Postgres");

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish();

    let app = axum::Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    println!("GraphiQL IDE: http://localhost:1234");

    axum::serve(TcpListener::bind("127.0.0.1:1234").await.unwrap(), app)
        .await
        .unwrap();
}
