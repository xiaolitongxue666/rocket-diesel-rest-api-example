use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::messages::{NewMessage, Message};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_message(conn: &PgConnection, contents: &str) -> Message {
    use crate::schema::messages;

    let new_message = NewMessage { contents };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .get_result(conn)
        .expect("Error saving new message")
}