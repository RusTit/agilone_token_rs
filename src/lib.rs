use rocket::get;

#[get("/")]
pub fn greeting() -> String {
    "Hi".into()
}

#[get("/<name>/<age>")]
pub fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}
