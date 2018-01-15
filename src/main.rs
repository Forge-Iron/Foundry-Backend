

#[macro_use]
extern crate diesel;


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate rustc_serialize;
extern crate dotenv;
#[macro_use]
extern crate rouille;

mod schema;
mod models;
mod handlers;
mod database;


use rouille::Request;
use rouille::Response;

fn main() {
    println!("Starting server on port: 8000");
    rouille::start_server("0.0.0.0:8000", move |request| dispatch(&request));
}

fn dispatch(req: &Request) -> Response {

    router!(req, 
            (GET) (/get_issues) => {

                handlers::get_issues(req)
            },

            _ => Response::empty_400()
            )

}
