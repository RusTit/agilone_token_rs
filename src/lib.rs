use std::collections::HashMap;
use rocket::get;

#[get("/")]
pub async fn greeting() -> String {
    let resp = reqwest::get("https://httpbin.org/ip").await;
    if let Ok(res) = resp {
        // let data = res.json()::<HashMap<String, String>>().await;
        let data: HashMap<String, String> = res.json().await.unwrap();
        return format!("{:#?}", data)
    }
    "Hi".into()
}

#[get("/<name>/<age>")]
pub fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}
