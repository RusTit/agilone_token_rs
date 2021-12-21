#[macro_use]
extern crate rocket;
use agilone_token_rs::hello;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![hello])
}