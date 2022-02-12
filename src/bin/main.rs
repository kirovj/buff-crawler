use std::collections::HashMap;

use futures::future::join_all;

// use buff_crawler::Item;

async fn request(url: String) -> Result<String, reqwest::Error> {
    let mut result = String::new();

    match reqwest::get(&url).await {
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

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let urls = vec![
        "http://127.0.0.1:8000/test".to_string(),
        "http://127.0.0.1:8000/test".to_string(),
        "http://127.0.0.1:8000/test".to_string(),
        "http://127.0.0.1:8000/test".to_string(),
    ];

    let results = join_all(urls.into_iter().map(|url| request(url))).await;

    for r in results.into_iter() {
        if r.is_ok() {
            println!("{}", r.unwrap());
        }
    }

    Ok(())
}
