use rocket::get;
use std::collections::HashMap;

#[get("/")]
pub async fn greeting() -> String {
    let resp = reqwest::get("https://httpbin.org/ip").await;
    if let Err(e) = resp {
        return format!("API request failed: {}", e);
    }
    let res = resp.unwrap();
    let data = res.json::<HashMap<String, String>>().await;
    if let Err(e) = data {
        return format!("Parsing error: {}", e);
    }
    let data = data.unwrap();
    return format!("{:#?}", data);
}

#[get("/<name>/<age>")]
pub fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}
