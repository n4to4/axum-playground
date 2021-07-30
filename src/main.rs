use axum::{extract::Json, prelude::*};
use http::StatusCode;
use serde::Deserialize;
use serde_json::{json, Value};
use std::net::SocketAddr;

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

use axum::service;
use http::Response;
use tower::service_fn;
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() {
    //let app = route("/", get(root))
    //    .route("/users", post(create_user))
    //    .route("/text", get(text))
    //    .route("/string", get(string))
    //    .route("/404", get(not_found))
    //    .route("/html", get(html))
    //    .route("/json", get(json))
    //    .route("/hc", get(|| async { "Ok" }));

    let app = route(
        "/",
        service::any(service_fn(|_: Request<Body>| async {
            let res = Response::new(Body::from("Hi from GET /"));
            Ok::<_, std::io::Error>(res)
        })),
    )
    .route(
        "/static/Cargo.toml",
        service::get(ServeFile::new("Cargo.toml")),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        //.serve(app.into_make_service())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
