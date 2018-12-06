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

use rocket::local::Client;
use rocket_contrib::json::Json;
use message::{Status, StatusFE};
use dotenv::dotenv;
use std::env;

#[get("/status")]
fn status () -> Json<StatusFE> {
    let hostname = env::var("HOSTNAME")
        .unwrap_or("untitled".to_string());

    let stream = match env::var("EUPHEMUS_HOST") {
        Ok(host) => do_thing(host),
        Err(_) => None
    };

    Json(StatusFE {
        name: "k8sr0".to_string(),
        hostname: hostname,
        is_up: true,
        leaf: stream.into_iter().collect()
    })
}

fn do_thing(host: String) -> Option<Status> {
    let client = Client::new(rocket::ignite()).expect("valid rocket");
    let req = client.get(format!("http://{}/status", host))
        .dispatch()
        .body_string();

    match req {
        Some(s) => {
            warn!("request fulfilled: {}", s);
            serde_json::from_str(s.as_str()).ok()
        },
        None => {
            warn!("request not fulfilled");
            None
        }
    }
}

#[get("/")]
fn index () -> String {
    "ha ha".to_string()
}

fn main () {
    env_logger::init();
    dotenv().ok();

    if let Err(ref e) = app::run(|_| routes![status, index]) {
        error!("error: {}", e);

        ::std::process::exit(1);
    }
}