
use dotenv::dotenv;

use diesel::prelude::*;
use std::env;

pub fn get_database_connection() -> SqliteConnection {

    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL");
    SqliteConnection::establish(&url).expect("Couldn't open database connection!")
}
