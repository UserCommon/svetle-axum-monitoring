extern crate dotenv;

use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Router,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize)]
struct Text(String);

#[tokio::main]
async fn main() {
    dotenv().ok();
    let router = Router::new()
        .route("/api/sys", get(system))
        .route("/", get(root));

    let bindy: String = format!(
        "{}:{}",
        std::env::var("IP").expect("IP must be set in .env file!"),
        std::env::var("PORT").expect("PORT must be set in .env file!")
    );

    axum::Server::bind(&&bindy.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Json<Text> {
    Json(Text("Hey!".to_string()))
}

async fn system() {}
