#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket::routes;
use std::env;
use std::fs::{create_dir, read_dir};
mod cors;
mod mnstr;

/// hello can be used for quick uptime checks
#[get("/")]
fn hello() -> &'static str {
    "Hello, fellow mnstr!"
}

#[rocket::launch]
fn rocket() -> _ {
    // Make sure we have an asset storage created
    let path_base = env::var("MNSTR_STRG").unwrap_or(String::from("/tmp/mnstr"));
    match read_dir(path_base.clone()) {
        Ok(_dir) => {},
        Err(_err) => {
            create_dir(path_base.clone()).unwrap()
        }
    }

    // Configure rocket for launch
    rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![hello])
        .mount("/assets/", FileServer::from(path_base))
}
