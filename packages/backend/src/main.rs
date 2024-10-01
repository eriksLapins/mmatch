use std::sync::Arc;

use axum::{Router, routing::get, routing::post, Json};
use db::{create_user, AppState};
use user::{Band, Manager, Musician, User};
mod user;
pub mod schema;
pub mod db;

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState);
    let router = Router::new()
        .route("/user", get(Json(User::default())))
        .route("/manager", get(Json(Manager::default())))
        .route("/musician", get(Json(Musician::default())))
        .route("/band", get(Json(Band::default())))
        .route("/user/create", post({
            let shared_state = Arc::clone(&shared_state);
            move |body| create_user(body, shared_state)
        }));


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3002").await.unwrap();
    axum::serve(listener, router.into_make_service()).await.unwrap();
}
