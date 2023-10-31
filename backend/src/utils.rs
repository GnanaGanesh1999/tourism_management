use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use uuid::Uuid;

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn generate_id() -> i32 {
    Uuid::new_v4().as_u128() as i32
}