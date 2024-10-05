use std::{fmt::Debug, sync::Arc};

use axum::{http::{HeaderValue, Response, header}, middleware::map_response, routing::{get, post}, Json, Router};
use db::AppState;
use user::{Band, Manager, Musician, User};
use tower_http::cors::CorsLayer;
mod user;
pub mod schema;
pub mod db;
pub mod errors;

async fn header_layer<T: Debug>(mut response: Response<T>) -> Response<T> {
    let headers = response.headers_mut();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    response
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState);
    let router = Router::new()
    .route("/manager", get(Json(Manager::default())))
    .route("/musician", get(Json(Musician::default())))
    .route("/band", get(Json(Band::default())))
    .route("/user", get({
        let shared_state = Arc::clone(&shared_state);
        || User::get_all(shared_state)
    }))
    .route("/user/create", post({
        let shared_state = Arc::clone(&shared_state);
        move |body| User::create(body, shared_state)
    }))
    .route("/user/:user", get({
        let shared_state = Arc::clone(&shared_state);
        |path| User::get(path, shared_state)
    }))
    .layer(
        CorsLayer::new()
            .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap())
            .allow_headers([header::CONTENT_TYPE])
    )
    .layer(map_response(header_layer));
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3002").await.unwrap();
    axum::serve(listener, router.into_make_service()).await.unwrap();
}
