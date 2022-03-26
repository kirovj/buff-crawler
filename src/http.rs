use std::{collections::HashMap, time::Duration};

use crate::utils::UA;

lazy_static! {
    static ref AGENT: ureq::Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .user_agent(UA)
        .build();
}

enum Method {
    Get,
    PostJson,
}

fn request(
    url: &str,
    method: Method,
    data: Option<&HashMap<&str, &str>>,
) -> Result<String, ureq::Error> {
    let mut times = 1;
    loop {
        let r = match method {
            Method::Get => AGENT.get(url).call()?,
            Method::PostJson => AGENT
                .post(url)
                .set("content-type", "application/json")
                .send_string(serde_json::to_string(data.unwrap()).unwrap().as_str())?,
        };
        if r.status() != 200 && times < 3 {
            times += 1;
            continue;
        }
        break r.into_string().or_else(|_| Ok("".to_string()));
    }
}

pub fn get(url: &str) -> Result<String, ureq::Error> {
    request(url, Method::Get, None)
}

pub fn post_json(url: &str, data: &HashMap<&str, &str>) -> Result<String, ureq::Error> {
    request(url, Method::PostJson, Some(data))
}
