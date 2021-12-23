extern crate reqwest;
use reqwest::{Client, Response};
use rocket::get;
use std::collections::HashMap;
use std::time::Duration;

const AUTH_DOMAIN: &str = "https://auth.agilone.com/token";
const MAX_TOKEN_AGE: u32 = 259200; // 3 days

struct agileOneOperation {
    create: bool,
}

async fn agil_one(operation_options: agileOneOperation) -> Response {
    let client = Client::new();
    let mut query = vec![("scheme", "a1webtag")];
    let client = if operation_options.create {
        query.push(("action", "create"));
        client.post(AUTH_DOMAIN)
    } else {
        client.get(AUTH_DOMAIN)
    };
    let timeout = Duration::from_secs(5);
    let resp = client
        .basic_auth("username", Some("password"))
        .query(&query)
        .timeout(timeout)
        .send()
        .await;
    resp.unwrap()
}

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
