mod schema;

use axum::{
    handler::{get, post},
    Router,
};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use schema::{create_schema, Schema};
use serde_json::json;
use std::net::SocketAddr;
use tokio::sync::Arc;

#[tokio::main]
async fn main() {
    let schema = Arc::new(create_schema());
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphiql", get(graphiql_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Server started at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_handler(
    body: axum::extract::Json<GraphQLRequest>,
    schema: axum::extract::Extension<Arc<Schema>>,
) -> axum::response::Json<serde_json::Value> {
    let schema = schema.0;
    let request = body.0;
    let response = request.execute(&schema, &()).await;
    axum::response::Json(json!(response))
}

async fn graphiql_handler() -> axum::response::Html<String> {
    let html = graphiql_source("/graphql");
    axum::response::Html(html)
}