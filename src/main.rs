#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::{Json, JsonValue};
use rocket::http::RawStr;

#[get("/")]
fn index() -> JsonValue {
    json!({
        "status": "success",
        "message": "Bonjour Yuana!"
    })
}

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> JsonValue {
    json!({
        "status": "success",
        "message": format!("Hello, {}!", name.as_str())
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "message": "Resource was not found."
    })
}

fn server() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, hello])
        .register(catchers![not_found])
}

fn main() {
    server().launch();
}