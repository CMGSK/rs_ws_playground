use axum::{Json, Router};
use serde::Deserialize;
use axum::routing::post;
use serde_json::{json, Value};
use crate::{web_layer, Error, Result};

pub fn routes() -> Router {
    return Router::new().route("/api/login/", post(api_login));
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!(" > Login");

    if payload.username != "miku" || payload.pwd != "beam" {
        return Err(Error::LoginFail);
    }

    let body = Json(json!({
        "result": "succesfully logged-in"
    }));

    return Ok(body);
}


#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String
}