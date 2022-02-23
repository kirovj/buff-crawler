#[macro_use]
extern crate lazy_static;

mod constant;
mod db;
mod http;
mod item;

use item::{PriceInfo, Category};
use constant::{CATEGORY_FILE, DB_FILE};

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

struct Crawler {
    db_helper: DbHelper,
}

impl Crawler {
    fn new(db_file: &str) -> Crawler {
        let db_helper = DbHelper::new(db_file);
        Crawler { db_helper }
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
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
                match request(self.build_url(name, page).as_str()) {
                    Ok(r) => match &serde_json::from_str(r.as_str()) {
                        Ok(v) => if page > self.process(v) {
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

    fn process(&self, value: &Value) -> u8 {
        let data = &value["data"];
        match &data["total_page"].as_u64() {
            Some(p) => {
                match &data["items"].as_array() {
                    Some(items) => {
                        items.into_iter().map(|item| self.process_item(item));
                    }
                    _ => {}
                }
                *p as u8
            }
            _ => 0
        }
    }

    fn process_item(&self, value: &Value) {
        let info = &value["goods_info"]["info"]["tags"];
        let exterior = &info["exterior"];
        let quality = &info["quality"];
        let rarity = &info["rarity"];
        let typo = &info["typo"];
        let weapon = &info["weapon"];
        let item = Item::new("".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), false, );
        match self.db_helper.get_item_id(&item) {
            None => {},
            Some(id) => {
                let date = Local::now().format("%Y-%m-%d").to_string();
                let price = &value["sell_min_price"].as_f64().unwrap();
                self.db_helper.add_price_info(&PriceInfo::new(id, date, *price as f32));
            }
        };
    }

    fn build_url(&self, category: &str, page: u8) -> String {
        let mut api = String::from(API);
        api.push_str("&page_num=");
        api.push_str(page.to_string().as_str());
        api.push_str("&category=");
        api.push_str(category);
        api
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let crawler = Crawler::new(DB_FILE);
    crawler.run();
    Ok(())
}
