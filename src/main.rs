use actix_web::{delete, get, put, post, App, HttpResponse, HttpServer, web};
use std::cell::RefCell;
use std::io::Result;

struct Item {
    name: String,
    id: i32
}

thread_local! {
    static LIST: RefCell<Vec<Item>> = RefCell::new(Vec::new());
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

#[post("/add")]
async fn add_item(item:String) -> Result<HttpResponse> {
    LIST.with(|list| {
        let mut list = list.borrow_mut();
        let new_item = Item {
            name: item.clone(),
            id: list.len() as i32
        };
        list.push(new_item);
    });

    Ok(HttpResponse::Ok().body(LIST.with(|list| {})))
}

#[delete("/id")]
async fn delete_item(id: web::Path<i32>) -> Result<HttpResponse> {
    let id = id.into_inner();
    LIST.with(|list| {
        let mut list = list.borrow_mut();
        for l in list.iter_mut() {
            if l.id == id {
                list.remove(l.id as usize);
            }
        }
    });

    Ok(HttpResponse::Ok().body(LIST.with(|list| {})))
}

#[actix_web::main]
async fn main() -> Result<()> {

    HttpServer::new(|| {
        App::new().service(hello) // Register the `hello` handler.
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}