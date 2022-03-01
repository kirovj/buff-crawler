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

pub fn request(url: &str) -> Result<String, reqwest::Error> {
    let mut times = 1;
    loop {
        let r = CLIENT.get(url).send()?.text();
        if !r.is_ok() && times < 3 {
            times += 1;
            continue;
        }
        break r;
    }
}
