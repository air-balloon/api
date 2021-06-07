#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use crate::dotenv::dotenv;
use std::env;

mod db;
mod models;
mod ressources;
mod routes;
mod schema;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    rocket::ignite()
        .manage(db::establish_connection(database_url))
        .mount("/", routes![routes::api::index::index])
        .launch();
}
