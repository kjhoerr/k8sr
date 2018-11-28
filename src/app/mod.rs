
pub mod error;

use rocket::Route;
use dotenv::dotenv;
use failure::Error;
use gauc::client::*;
use std::env;
use self::error::CouchbaseError;

pub fn run<R: Into<Vec<Route>>, F>(routes: F) -> Result<(), Error>
where F: Fn(Client) -> R {
    dotenv().ok();

    let cb_conn = &get_cb_conn()?;
    let cb_auth = get_cb_auth().ok();
    let couchbase = Client::connect(cb_conn, cb_auth)
        .map_err(CouchbaseError::new)?;

    rocket::ignite().mount("/", routes(couchbase)).launch();

    Ok(())
}

fn get_cb_conn () -> Result<String, Error> {
    let cb_host = env::var("COUCHBASE_HOST")
        .expect("COUCHBASE_HOST must be set");
    let cb_bckt = env::var("COUCHBASE_BUCKET")
        .expect("COUCHBASE_BUCKET must be set");

    let cb_conn = format!("couchbase://{}/{}", cb_host, cb_bckt);

    info!("Couchbase server configured at {}", cb_conn);

    Ok(cb_conn)
}

fn get_cb_auth () -> Result<Authenticator, Error> {
    let cb_user = env::var("COUCHBASE_USERNAME")
        .expect("COUCHBASE_USERNAME must be set");
    let cb_pass = env::var("COUCHBASE_PASSWORD")
        .expect("COUCHBASE_PASSWORD must be set");

    let mut auth = Authenticator::new(AuthType::Rbac);
    auth.add_password(cb_user, cb_pass, AuthFlags::Bucket);

    Ok(auth)
}
