#![feature(proc_macro_hygiene, decl_macro)]

extern crate dotenv;
extern crate env_logger;
extern crate failure;
#[macro_use] extern crate log;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate gauc;

pub mod app;
pub mod message;

use rocket_contrib::json::Json;
use message::Status;
use dotenv::dotenv;
use std::env;

#[get("/status")]
fn status () -> Json<Status> {
    let hostname = env::var("HOSTNAME")
        .unwrap_or("untitled".to_string());

    Json(Status {
        name: "k8sr1".to_string(),
        hostname: hostname,
        is_up: true
    })
}

fn main () {
    env_logger::init();
    dotenv().ok();

    if let Err(ref e) = app::run(|_| routes![status]) {
        error!("error: {}", e);

        ::std::process::exit(1);
    }
}