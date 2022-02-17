#[macro_use]
extern crate lazy_static;

// mod db;
mod http;
mod item;

use futures::future::join_all;
use item::PriceInfo;
use serde_json::Value;

use std::error::Error;

fn deal_response(value: &Value) {
    let code = &value["code"].as_str();
    match *code {
        Some(c) => {
            println!("{}", c)
        }
        _ => {}
    }
    let data = &value["data"];
    let items = &data["items"].as_array().unwrap();
    let page_num = &data["page_num"].as_u64().unwrap();
    let total_page = &data["total_page"].as_u64().unwrap();

    for item in items.into_iter() {
        println!("{:?}", build_price_info(item).unwrap());
        println!("------");
    }

    println!("page_num: {:?}", page_num);
    println!("total_page: {:?}", total_page);
}

fn build_price_info(value: &Value) -> Result<PriceInfo, Box<dyn Error>> {
    let price: f32 = value["sell_min_price"]
        .as_str()
        .unwrap()
        .to_owned()
        .parse()?;
    let id = 0;
    let item_id = 1;
    let date = "2022-02-17".to_string();
    Ok(PriceInfo::new(id, item_id, date, price))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let urls = vec![
        "https://buff.163.com/api/market/goods?game=csgo&page_num=1&category=weapon_knife_survival_bowie&use_suggestion=0&trigger=undefined_trigger",
    ];

    let results = join_all(urls.into_iter().map(|url| http::request(url))).await;

    for r in results.into_iter() {
        match r {
            Ok(value) => {
                deal_response(&value);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(())
}
