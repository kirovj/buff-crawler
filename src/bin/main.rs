use std::thread;

use reqwest::blocking::{Client, Response};

// use std::collections::HashMap;

// use buff_crawler::Item;

fn request(c: &Client) -> Response {
    c.get("http://127.0.0.1:8000/test")
        .header("user-agent", "jack")
        .send()
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let t = thread::spawn(move || {
        for i in 1..3 {
            let res = request(&client);
            let text = res.text();
            println!("No.{}, text: {:?}", i, text);
        }
    });
    t.join().unwrap();
    Ok(())
}
