mod schema;

use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post},
    response::Html,
    Router,
};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use schema::{create_schema, Schema};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

#[tokio::main]
async fn main() {
    let schema = Arc::new(create_schema());
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphiql", get(graphiql_handler))
        .layer(AddExtensionLayer::new(schema));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Server started at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_handler(
    Json(request): Json<GraphQLRequest>,
    Extension(schema): Extension<Arc<Schema>>,
) -> axum::Json<serde_json::Value> {
    let response = request.execute(&schema, &()).await;
    axum::Json(json!(response))
}

async fn graphiql_handler() -> Html<String> {
    let html = graphiql_source("/graphql", Some("ws://localhost:8080/subscriptions"));
    Html(html)
}
