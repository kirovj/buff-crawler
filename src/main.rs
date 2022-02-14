// use std::collections::HashMap;
mod http;
mod item;

// use crate::http;
use futures::future::join_all;

// use buff_crawler::Item;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let urls = vec![
        "http://127.0.0.1:8000/test".to_string(),
        "http://127.0.0.1:8000/test".to_string(),
        "http://127.0.0.1:8000/test".to_string(),
        "http://127.0.0.1:8000/test".to_string(),
    ];

    let results = join_all(urls.into_iter().map(|url| http::request(url))).await;

    for r in results.into_iter() {
        if r.is_ok() {
            println!("{}", r.unwrap());
        }
    }

    Ok(())
}
