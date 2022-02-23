#[macro_use]
extern crate lazy_static;

mod constant;
mod db;
mod http;
mod item;

use item::{PriceInfo, Category};
use constant::CATEGORY_FILE;

use serde_json::Value;

use std::error::Error;
use std::fs;
use std::{thread, time};
use chrono::Local;
use regex::Regex;
use crate::constant::API;
use crate::http::request;
use crate::item::Item;
use crate::db::DbHelper;

lazy_static! {
    static ref CATEGORY: Category = Category::from_json(CATEGORY_FILE);
}

fn process(value: &Value) -> u8 {
    let data = &value["data"];
    match &data["total_page"].as_u64() {
        Some(p) => {
            match &data["items"].as_array() {
                Some(items) => {
                    items.into_iter().map(|item| process_item(item));
                }
                _ => {}
            }
            *p as u8
        }
        _ => 0
    }
}

fn process_item(v: &Value) {
    match build_item(v) {
        Ok(item) => {}
        _ => {}
    }
}

fn build_item(value: &Value) -> Result<(Item, PriceInfo), Box<dyn Error>> {
    let item_id = 1;
    let date = Local::now().format("%Y-%m-%d").to_string();
    let price = &value["sell_min_price"].as_f64().unwrap();
    let price_info = PriceInfo::new(0, item_id, date, *price as f32);

    let info = &value["goods_info"]["info"]["tags"];
    let exterior = &info["exterior"];
    let quality = &info["quality"];
    let rarity = &info["rarity"];
    let typo = &info["typo"];
    let weapon = &info["weapon"];

    let item = Item {
        id: 0,
        name: "".to_string(),
        class: "".to_string(),
        typo: "".to_string(),
        ware: "".to_string(),
        quality: "".to_string(),
        rarity: "".to_string(),
        stat_trak: false,
    };

    Ok((item, price_info))
}

fn build_url(category: &str, page: u8) -> String {
    let mut api = String::from(API);
    api.push_str("&page_num=");
    api.push_str(page.to_string().as_str());
    api.push_str("&category=");
    api.push_str(category);
    api
}

fn main() -> Result<(), Box<dyn Error>> {
    let url_entrance = "https://buff.163.com/market/csgo";

    let html = request(url_entrance)?;
    let re = Regex::new("<li value=\"([^\"]+)\">([^<]*)</li>")?;
    let items = re.captures_iter(html.as_str()).filter_map(|cap| {
        match (cap.get(1), cap.get(2)) {
            (Some(name), Some(name_zh)) => Some((name.as_str(), name_zh.as_str())),
            _ => None,
        }
    }).collect::<Vec<(&str, &str)>>();

    for (name, name_zh) in items {
        println!("start crawl {}|{}", name, name_zh);
        let mut page: u8 = 1;
        loop {
            match request(build_url(name, page).as_str()) {
                Ok(r) => match &serde_json::from_str(r.as_str()) {
                    Ok(v) => if page > process(v) {
                        break;
                    },
                    _ => break,
                },
                _ => break,
            };
            thread::sleep(time::Duration::from_secs(3));
            page += 1;
        }
        break;
    }

    Ok(())
}
