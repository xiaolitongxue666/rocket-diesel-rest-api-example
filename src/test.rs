use crate::rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

#[test]
fn post_get_put_get() {
    // Create a new rocket client
    let client = Client::new(rocket()).unwrap();

    ///////////////////////////////////////////////////////////

    // Add a new message with ID 1.
    let res = client.post("/message/1")
        .header(ContentType::JSON)
        .body(r#"{ "contents": "Hello, world!" }"#)
        .dispatch();

    assert_eq!(res.status(), Status::Ok);

    ///////////////////////////////////////////////////////////

    // Check that the message exists with the correct contents.
    let mut res = client.get("/message/1").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body_string().unwrap();
    assert!(body.contains("Hello, world!"));

    ///////////////////////////////////////////////////////////

    // Update post data
    // Change the message contents.
    let res = client.put("/message/1")
        .header(ContentType::JSON)
        .body(r#"{ "contents": "Bye bye, world!" }"#)
        .dispatch();

    assert_eq!(res.status(), Status::Ok);
}