use std::{fmt::Debug, sync::Arc};

use axum::{http::{HeaderValue, Response, header}, middleware::map_response, routing::{get, post}, Router};
use tower_http::cors::CorsLayer;
pub mod schema;
pub mod db;
pub mod errors;
pub mod models;
pub mod utils;

mod prelude {
    pub use diesel::prelude::*;
    pub use crate::models::user::* ;
    pub use crate::models::musician::*;
    pub use crate::models::band::*;
    pub use crate::models::manager::*;
    pub use crate::db::*;
    pub use serde::{Deserialize, Serialize};
    pub use ts_rs::TS;
    pub use serde_json::Value;
    pub use axum::{body::Body, extract::Path, http::{Response, StatusCode}, Json};
    pub use std::sync::Arc;
    pub use crate::schema::*;
    pub use crate::errors;
    pub use crate::utils::*;
}

use crate::prelude::*;

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
    .route("/musician/create", post({
        let shared_state = Arc::clone(&shared_state);
        move |body| Musician::create(body, shared_state)
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
