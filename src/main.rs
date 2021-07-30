use axum::{extract::Json, prelude::*};
use serde::Deserialize;
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

#[tokio::main]
async fn main() {
    let app = route("/", get(root)).route("/users", post(create_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
