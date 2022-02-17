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

pub async fn request(url: String) -> Result<String, reqwest::Error> {
    let mut result = String::new();

    let client = reqwest::Client::builder()
        .default_headers(make_headers())
        .build()?;

    match client.get(&url).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                result = text;
            }
            Err(_) => {
                println!("read response text error");
            }
        },
        Err(_) => println!("request get error"),
    }
    Ok(result)
}
