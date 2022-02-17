use reqwest::header::{HeaderMap, HeaderValue};

use std::fs;

const UA: &str = "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36";

lazy_static! {
    static ref COOKIE: String = fs::read_to_string("cookie.txt").unwrap();
}

fn make_headers() -> HeaderMap {
    let mut map = HeaderMap::new();
    map.insert("cookie", HeaderValue::from_static(COOKIE.as_str()));
    map.insert("user-agent", HeaderValue::from_static(UA));
    map
}

pub async fn request(url: String) -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::builder()
        .default_headers(make_headers())
        .build()?;

    let result: serde_json::Value = client.get(&url).send().await?.json().await?;
    Ok(result)
}
