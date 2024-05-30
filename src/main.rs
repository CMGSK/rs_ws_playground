use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router
};
use serde::Deserialize;
use serde_json::json;
use tower::ServiceBuilder;
use std::{net::{SocketAddr}, sync::{Arc, RwLock}};

//Listen for re-compilations: cargo watch -q -c -w src/ -x run

#[tokio::main]
async fn main(){

    let app = Router::new()
        .route("/", get(handler_helloworld));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("Listening.....");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

}

#[derive(Debug, Deserialize)]
struct HelloWorld_parameters {
    name: Option<String>
}

async fn handler_helloworld(Query(params): Query<HelloWorld_parameters>) -> impl IntoResponse{
    println!(" > handler_helloworld execution");

    let name = params.name.as_deref().unwrap_or("world!");
    return Html(format!("<h1>Hello, {name}</h1>"));
}
