#[macro_use]
extern crate rocket;
use agilone_token_rs::{greeting, hello};
use std::env;

#[launch]
fn rocket() -> _ {
    let port = env::var("PORT").unwrap_or("3000".into());
    let port = port.parse::<u32>().unwrap();
    let figment = rocket::Config::figment().merge(("port", port));
    rocket::custom(figment)
        .mount("/hello", routes![hello])
        .mount("/", routes![greeting])
}
