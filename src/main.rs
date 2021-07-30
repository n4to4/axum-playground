use axum::{extract::Json, prelude::*, service};
use http::{Response, StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use std::convert::Infallible;
use std::net::SocketAddr;
use tower::service_fn;

async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
}

async fn create_user(Json(payload): Json<CreateUser>) {
    dbg!(&payload);
}

async fn text() -> &'static str {
    "text"
}

async fn string() -> String {
    "string".to_owned()
}

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "not found")
}

async fn html() -> response::Html<&'static str> {
    response::Html("<h1>hello world</h1>")
}

async fn json() -> response::Json<Value> {
    response::Json(json!({"data":42}))
}

#[tokio::main]
async fn main() {
    let app = route("/", get(root))
        .route("/users", post(create_user))
        .route("/text", get(text))
        .route("/string", get(string))
        .route("/404", get(not_found))
        .route("/html", get(html))
        .route("/json", get(json))
        .route(
            "/root",
            service::any(service_fn(|_: Request<Body>| async {
                let res = Response::new(Body::from("Hi from GET /root"));
                Ok::<_, Infallible>(res)
            })),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
