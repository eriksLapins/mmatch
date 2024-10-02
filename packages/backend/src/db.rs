use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("No DATABASE_URL provided in .env");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to: {}", database_url))
}

pub struct AppState;