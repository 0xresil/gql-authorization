mod auth;
mod schema;
use auth::AuthInfo;
use axum::{
    extract::{Extension, Json, Path},
    response::Html,
    routing::{get, post},
    Router,
};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use schema::{create_schema, Schema};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::add_extension::AddExtensionLayer;

fn get_router() -> Router {
    println!("get_router");
    let schema = Arc::new(create_schema());
    Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphiql", get(graphiql_handler))
        .layer(AddExtensionLayer::new(schema))
}

#[tokio::main]
async fn main() {
    let app = get_router();

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
    auth_info: AuthInfo,
) -> Result<axum::Json<serde_json::Value>, String> {
    println!("request arrived {:?}", request);
    let response = request.execute(&schema, &auth_info).await;
    Ok(axum::Json(json!(response)))
}

async fn graphiql_handler() -> Html<String> {
    let html = graphiql_source("/graphql", Some("ws://localhost:8080/subscriptions"));
    Html(html)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use axum::{
        body::Bytes,
        http::{
            header::{AUTHORIZATION, CONTENT_TYPE},
            Method, Request,
        },
    };
    use http_body::combinators::UnsyncBoxBody;
    use serde::{de::DeserializeOwned, Serialize};
    use tower::ServiceExt;

    pub async fn send_request(
        router: &Router,
        request: Request<hyper::Body>,
    ) -> hyper::Response<UnsyncBoxBody<Bytes, axum::Error>> {
        router
            .clone()
            .oneshot(request)
            .await
            .expect("failed to send oneshot request")
    }

    pub async fn get(
        router: &Router,
        uri: impl AsRef<str>,
    ) -> hyper::Response<UnsyncBoxBody<Bytes, axum::Error>> {
        let request = Request::builder()
            .method(Method::GET)
            .uri(uri.as_ref())
            .body(hyper::Body::empty())
            .expect("failed to build GET request");
        send_request(router, request).await
    }

    pub async fn post<T: Serialize>(
        router: &Router,
        uri: impl AsRef<str>,
        body: &T,
    ) -> hyper::Response<UnsyncBoxBody<Bytes, axum::Error>> {
        let request = Request::builder()
            .method(Method::POST)
            .uri(uri.as_ref())
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, "Bearer test_token")
            .body(
                serde_json::to_vec(body)
                    .expect("failed to serialize POST body")
                    .into(),
            )
            .expect("failed to build POST request");
        send_request(router, request).await
    }

    pub async fn deserialize_response_body<T>(
        response: hyper::Response<UnsyncBoxBody<Bytes, axum::Error>>,
    ) -> T
    where
        T: DeserializeOwned,
    {
        let bytes = hyper::body::to_bytes(response.into_body())
            .await
            .expect("failed to read response body into bytes");
        serde_json::from_slice::<T>(&bytes).expect("failed to deserialize response")
    }

    #[tokio::test]

    async fn try_post() {
        const uri: &'static str = "https://localhost:8000";
        let request_body = json!({
          "query": r#"
            {
              user(id: "1") {
                id,
                username
              }
            }
          "#
        });
        let response = post(&get_router(), "/graphql", &request_body).await;

        println!("response {:?}", response);
        assert_eq!(response.status(), 200);

        println!(
            "response_body {:?}",
            deserialize_response_body::<serde_json::Value>(response).await
        );
    }
}
