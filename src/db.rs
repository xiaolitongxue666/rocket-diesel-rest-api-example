use crate::messages::{NewMessage, Message};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::schema::messages::dsl::messages;

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

pub fn show_messages() {
    // use crate::schema::messages::dsl::messages;

    let connection = establish_connection();
    let result = messages.limit(5)
        .load::<Message>(&connection)
        .expect("Error loading messages");

    println!("Display {} messages",result.len());
    for message in result {
        println!("------------------------------------");
        println!("id:{}     |     content:{}", message.id, message.contents);
        println!("------------------------------------\n");
    }
}

pub fn delete_message(id: i32) -> usize {

    let connection = establish_connection();

    let num_deleted = diesel::delete(messages.find(id))
        .execute(&connection)
        .expect("Error deleting messages");

    println!("Deleted {} messages", num_deleted);
    num_deleted
}