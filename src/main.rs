#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;

use dotenv::dotenv;

mod backend;
mod connection;
mod schema;

/*
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}*/

fn main() {
    dotenv().ok();
    backend::router_da01::create_routes();
    //rocket::ignite().mount("/", routes![index]).launch();
}
