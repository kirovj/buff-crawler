// use std::collections::HashMap;

// use buff_crawler::Item;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get("https://httpbin.org/headers")
        .header("user-agent", "jack")
        .send()?
        .text()?;
    println!("{:?}", resp);
    Ok(())
}
