#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
// #[macro_use] extern crate dotenv;

#[cfg(test)] mod test;
mod messages;
mod schema;
mod db;

use rocket_contrib::json::{Json, JsonValue};
// use serde::{Serialize, Deserialize};
use crate::messages::Message;
// use std::collections::HashMap;

// The type to represent the ID of a message.
type ID = i32;

// #[derive(Serialize, Deserialize)]
// struct Message {
//     id: Option<ID>,
//     contents: String
// }
// pub mod messages;
// pub mod db;

#[post("/<id>", format = "json", data = "<message>")]
fn new(id: ID, message: Json<Message>) -> JsonValue {
    if false {
        //if sql have this post , return a error message
        json!({
            "status": "error",
            "reason": "ID exists. Try put."
        })
    } else {
        //if sql don't have this post , create it and return success message
        println!("Create a new post in sql like : id[{}] , message[{}]" , id, message.0.contents);
        json!({ "status": "ok" })
    }
}

#[put("/<id>", format = "json", data = "<message>")]
fn update(id: ID, message: Json<Message>) -> Option<JsonValue> {
    if true {
        println!("Update a post in sql like : id[{}] , message[{}]" , id, message.0.contents);
        Some(json!({ "status": "ok" }))
    } else {
        None
    }
}

#[get("/<id>", format = "json")]
fn get(id: ID) -> Json<Message> {
    let contents = "Hello, world!";
    Json(Message {
        id: id as i32,
        contents: contents.to_string(),
        // published: false
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

// #[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/message", routes![new, update, get])
        .register(catchers![not_found])
}

fn main(){
    println!("Hello rocket");
}