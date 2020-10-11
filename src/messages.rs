use serde::{Serialize, Deserialize};
use super::schema::messages;

// The type to represent the ID of a message.
// type ID = usize;

// #[derive(Serialize, Deserialize)]
#[derive(Serialize, Deserialize, Queryable)]
pub struct Message {
    pub id: i32,
    pub contents: String,
    // pub published: bool,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub contents: &'a str,
}