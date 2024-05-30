use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service, post},
    Json, Router
};
use serde::Deserialize;
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use std::{net::{SocketAddr}, sync::{Arc, RwLock}};

pub use self::error::{Error, Result};


mod error;
mod web_layer;
use web_layer::routes_login;
//Listen for re-compilations: cargo watch -q -c -w src/ -x run

#[tokio::main]
async fn main(){

    let app = Router::new()
        .merge(helloworlds())
        .merge(web_layer::routes_login::routes())
        .fallback_service(statics());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("Listening.....");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

}

fn helloworlds() -> Router {
    return Router::new()
        .route("/hello/", get(handler_helloworld_qry))
        .route("/hello/:name/", get(handler_helloworld_path));
}

fn statics() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct HelloWorld_parameters {
    name: Option<String>
}

async fn handler_helloworld_qry(Query(params): Query<HelloWorld_parameters>) -> impl IntoResponse{
    println!(" > handler_helloworld_qry execution");

    let name = params.name.as_deref().unwrap_or("world!");
    return Html(format!("<h1>Hello, {name}</h1>"));
}

async fn handler_helloworld_path(Path(name): Path<String>) -> impl IntoResponse {
    println!(" > handler_helloworld_path execution");

    return Html(format!("<h1>Hello, {name}</h1>"));
}

