use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::utils::UA;

fn make_headers() -> HeaderMap {
    let mut map = HeaderMap::new();
    map.insert("user-agent", HeaderValue::from_static(UA));
    map
}

lazy_static! {
    static ref CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .default_headers(make_headers())
        .build()
        .unwrap();
}

enum Method {
    Get,
    PostJson,
}

fn request(
    url: &str,
    method: Method,
    data: Option<HashMap<String, String>>,
) -> Result<String, reqwest::Error> {
    let mut times = 1;
    loop {
        let r = match method {
            Method::Get => CLIENT.get(url).send()?.text(),
            Method::PostJson => CLIENT.post(url).json(data.as_ref().unwrap()).send()?.text(),
        };
        if !r.is_ok() && times < 3 {
            times += 1;
            continue;
        }
        break r;
    }
}

pub fn get(url: &str) -> Result<String, reqwest::Error> {
    request(url, Method::Get, None)
}

pub fn post_json(url: &str, data: HashMap<String, String>) -> Result<String, reqwest::Error> {
    request(url, Method::Get, Some(data))
}
