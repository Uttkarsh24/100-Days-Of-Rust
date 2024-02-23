#[macro_use] extern crate rocket;

use rocket::serde::json::{ Value, json };
use rocket::response::status;

#[get("/rustaceans")]
fn get_rustaceans() -> Value {
    json!([{"id":1, "name":"John Doe"}, {"id":2, "name":"Johnson Doe"}])
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32) -> Value {
    json!({"id":id, "name":"Anmol Anand", "email":"aa@gmail.com"})
}

#[post("/rustaceans", format="json")]
fn create_rustacean() -> Value {
    json!({"id":3, "name":"Anmol Anand", "email":"aa@gmail.com"})
}

#[put("/rustaceans/<id>", format="json")]
fn update_rustacean(id: i32) -> Value {
    json!({"id":id, "name":"Anmol Anand", "email":"aa@gmail.com"})
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_rustaceans,
            view_rustacean,
            create_rustacean,
            update_rustacean,
            delete_rustacean
        ])
        .register("/", catchers![
            not_found
        ])
        .launch()
        .await;
}