#![feature(proc_macro_hygiene, decl_macro)]

extern crate dotenv;
extern crate env_logger;
extern crate failure;
#[macro_use] extern crate log;
#[macro_use] extern crate rocket;
extern crate gauc;

pub mod app;

#[get("/status")]
fn index () -> String {
    format!("k8sr0 server up")
}

fn main () {
    env_logger::init();

    if let Err(ref e) = app::run(|_| routes![index]) {
        error!("error: {}", e);

        ::std::process::exit(1);
    }
}