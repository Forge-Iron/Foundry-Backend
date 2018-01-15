

#[macro_use]
extern crate diesel;


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


extern crate dotenv;


mod schema;
mod models;
mod handlers;
mod database;


fn main() {
    println!("Hello, world!");
}
