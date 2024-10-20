use actix_web::{get, App, HttpResponse, HttpServer};
use std::cell::RefCell;
use std::io::Result;

thread_local! {
    static LIST: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[get("/")]
async fn hello() -> Result<HttpResponse> {
    // Access the `LIST` and convert it to a string.
    let list_data = LIST.with(|list| {
        let list = list.borrow(); // Borrow the list immutably.
        list.join(", ") // Join the elements into a single string.
    });

    Ok(HttpResponse::Ok().body(list_data)) // Return the list as the body.
}

#[actix_web::main]
async fn main() -> Result<()> {
    // Initialize some data in the LIST for testing.
    LIST.with(|list| {
        let mut list = list.borrow_mut(); // Borrow the list mutably.
        list.push("Item 1".to_string());
        list.push("Item 2".to_string());
        list.push("Item 3".to_string());
    });

    HttpServer::new(|| {
        App::new().service(hello) // Register the `hello` handler.
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}