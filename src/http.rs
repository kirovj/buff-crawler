use std::{collections::HashMap, time::Duration};

use crate::utils::UA;

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
    let agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .user_agent(UA)
        .build();
    loop {
        let r = match method {
            Method::Get => agent.get(url).call()?,
            Method::PostJson => agent
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

#[derive(Deserialize)]
struct Request {
    target: String,
    name: String,
    item_id: u32,
}

#[derive(Serialize)]
struct Response<T> {
    status: u8,
    message: String,
    data: Option<Vec<T>>,
}

impl<T: Serialize> Response<T> {
    fn ok(data: Vec<T>) -> Self {
        Response {
            status: 0,
            message: String::from("ok"),
            data: Some(data),
        }
    }

    fn ok_without_data() -> Self {
        Response {
            status: 0,
            message: String::from("ok"),
            data: None,
        }
    }

    fn fail(message: String) -> Self {
        Response {
            status: 1,
            message,
            data: None,
        }
    }

    fn new(result: Result<Vec<T>, Error>) -> Self {
        match result {
            Ok(data) => Self::ok(data),
            Err(e) => Self::fail(e.to_string()),
        }
    }
}

