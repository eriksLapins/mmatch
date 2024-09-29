use axum::{Router, routing::get, Json};
use user::{Band, Manager, Musician, User};
mod user;
pub mod schema;
pub mod db;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/user", get(Json(User::default())))
        .route("/manager", get(Json(Manager::default())))
        .route("/musician", get(Json(Musician::default())))
        .route("/band", get(Json(Band::default())))
        .route("/user/create", get(User::create(User::default())));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3002").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
