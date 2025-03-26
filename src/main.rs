use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use axum::{
    http::{header::HeaderValue, Response},
    response::{self, IntoResponse},
    routing::get,
    Extension, Json,
};
use graphql::{MutationRoot, QueryRoot};
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{info, Level};

pub mod graphql;
use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();
    // configure cors for testing, remove in production
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION, ACCEPT]);
    // setup database connection
    let pool = PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to Postgres");

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish();

    async fn graphql_handler(
        cookies: Cookies,
        Extension(schema): Extension<Schema<QueryRoot, MutationRoot, EmptySubscription>>,
        Json(request): Json<async_graphql::Request>,
    ) -> impl IntoResponse {
        let mut graphql_response = schema.execute(request.data(cookies)).await;
        let headers = std::mem::take(&mut graphql_response.http_headers);
        let mut res = Response::builder();
        for (key, value) in headers {
            res = res.header(key.unwrap(), value);
        }
        res.body(Json(graphql_response).into_response().into_body())
            .unwrap()
    }

    // Create a logging layer
    let _logging_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(
            |request: &http::Request<axum::body::Body>, _span: &tracing::Span| {
                info!(
                    "Received request: {} {} with headers: {:?}",
                    request.method(),
                    request.uri(),
                    request.headers()
                );
            },
        );

    let app = axum::Router::new()
        .route("/", get(graphiql).post(graphql_handler)) // Now extracts cookies first
        .layer(cors)
        .layer(CookieManagerLayer::new()) // Enables cookies
        .layer(Extension(schema)); // Inject schema

    println!("GraphiQL IDE: http://localhost:1234");

    axum::serve(TcpListener::bind("127.0.0.1:1234").await.unwrap(), app)
        .await
        .unwrap();
}
