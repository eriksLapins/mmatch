use axum::Json;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, sync::Arc};

use crate::user::{CreateUserPayload, User};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("No DATABASE_URL provided in .env");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to: {}", database_url))
}

pub struct AppState;


pub async fn create_user(Json(payload): Json<CreateUserPayload>, _state: Arc<AppState>) {

    let user = User::new(
        payload.name,
        payload.lastname,
        payload.description,
        payload.email,
        payload.phone,
        payload.phone_prefix,
        payload.country,
        payload.city,
        payload.street,
        payload.house_number,
        payload.apartment,
        payload.password,
        payload.types,
    );
    User::create(user).await
}