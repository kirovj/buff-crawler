use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::constant::{PROXY_FILE, UA};
use std::fs;

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
        serde_json::from_str(fs::read_to_string(PROXY_FILE).unwrap().as_str()).unwrap();
}

fn make_headers() -> HeaderMap {
    let mut map = HeaderMap::new();
    // map.insert("cookie", HeaderValue::from_static(COOKIE.as_str()));
    map.insert("user-agent", HeaderValue::from_static(UA));
    map
}

fn request_retry(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .default_headers(make_headers())
        .proxy(PROXY_PROVIDER.random())
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()?;
    client.get(url).send()?.text()
}

pub fn request(url: &str) -> Result<String, reqwest::Error> {
    let mut times = 1;
    loop {
        let r = request_retry(url);
        if !r.is_ok() && times < 3 {
            times += 1;
            continue;
        }
        break r;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_provider() {
        assert_eq!(9, PROXY_PROVIDER.addrs.len())
    }
}
