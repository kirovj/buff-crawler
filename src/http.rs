use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use std::fs;

const UA: &str = "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36";

#[derive(Serialize, Deserialize)]
struct ProxyProvider {
    user: String,
    pass: String,
    addrs: Vec<String>,
}

impl ProxyProvider {
    fn random(&self) -> reqwest::Proxy {
        let random: usize = rand::thread_rng().gen_range(0..self.addrs.len());
        let proxy = self.addrs[random].as_str();
        reqwest::Proxy::http(proxy)
            .unwrap()
            .basic_auth(self.user.as_str(), self.pass.as_str())
    }
}

lazy_static! {
    static ref PROXY_PROVIDER: ProxyProvider =
        serde_json::from_str(fs::read_to_string("proxies.json").unwrap().as_str()).unwrap();
    static ref COOKIE: String = fs::read_to_string("cookie.txt").unwrap();
}

fn make_headers() -> HeaderMap {
    let mut map = HeaderMap::new();
    map.insert("cookie", HeaderValue::from_static(COOKIE.as_str()));
    map.insert("user-agent", HeaderValue::from_static(UA));
    map
}

pub async fn request(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::builder()
        .default_headers(make_headers())
        .proxy(PROXY_PROVIDER.random())
        .build()?;

    let result: serde_json::Value = client.get(url).send().await?.json().await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_proxy_provider() {
        assert_eq!(9, PROXY_PROVIDER.addrs.len())
    }
}
