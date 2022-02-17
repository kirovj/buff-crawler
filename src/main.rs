#[macro_use]
extern crate lazy_static;

mod http;
mod item;

use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let urls = vec![
        "http://127.0.0.1:8000/headers".to_string(),
        "http://127.0.0.1:8000/headers".to_string(),
    ];

    let results = join_all(urls.into_iter().map(|url| http::request(url))).await;

    for r in results.into_iter() {
        if r.is_ok() {
            println!("{}", r.unwrap());
        }
    }

    Ok(())
}
