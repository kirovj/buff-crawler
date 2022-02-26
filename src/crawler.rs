use crate::constant::{API_BUFF, DEFAULT};
use crate::db::DbHelper;
use crate::http::request;
use crate::item::{Item, PriceInfo};

use chrono::Local;
use serde_json::Value;
use std::{thread, time};

pub trait Crawl {
    fn db(&self) -> &DbHelper;

    fn build_url(&self) -> String;

    fn fetch(&self) -> Option<String> {
        match request(self.build_url().as_str()) {
            Ok(html) => Some(html),
            _ => None,
        }
    }

    fn parse(&self, html: String);

    fn run(&self);

    fn persistent(&self, item: Item, price: usize) {
        match self.db().get_item_id(&item) {
            None => {}
            Some(id) => {
                let date = Local::now().format("%Y-%m-%d").to_string();
                self.db().add_price_info(&PriceInfo::new(id, date, price));
            }
        }
    }
}

pub enum Target {
    Buff,
    Yyyp,
    Igxe,
}

pub fn build_crawler(target: Target, db_file: &str) -> Box<dyn Crawl> {
    let db_helper = DbHelper::new(db_file);
    match target {
        Target::Buff => Box::new(BuffCrawler { db_helper }),
        Target::Yyyp => Box::new(YyypCrawler { db_helper }),
        Target::Igxe => Box::new(IgxeCrawler { db_helper }),
    }
}

pub struct BuffCrawler {
    db_helper: DbHelper,
}

pub struct YyypCrawler {
    db_helper: DbHelper,
}

pub struct IgxeCrawler {
    db_helper: DbHelper,
}

impl Crawl for BuffCrawler {
    fn db(&self) -> &DbHelper {
        &self.db_helper
    }

    fn build_url(&self) -> String {
        let mut url = String::from(API_BUFF);
        url.push_str(Local::now().timestamp_millis().to_string().as_str());
        url
    }

    fn parse(&self, html: String) {
        fn get_value(value: &Value, key: &str) -> String {
            value[key]["localized_name"]
                .as_str()
                .unwrap_or(DEFAULT)
                .to_string()
        }
        let value: Value = serde_json::from_str(html.as_str()).unwrap();
        let data = &value["data"];
        match data["items"].as_array() {
            Some(data_items) => {
                for data_item in data_items {
                    let info = &data_item["goods_info"]["info"]["tags"];
                    let item = Item::new(
                        data_item["short_name"].as_str().unwrap().to_string(),
                        get_value(info, "type"),
                        get_value(info, "weapon"),
                        get_value(info, "exterior"),
                        get_value(info, "quality"),
                        get_value(info, "rarity"),
                        get_value(info, "quality").contains("StatTrak"),
                    );
                    if let Some(price) = &data_item["sell_min_price"].as_str() {
                        println!("process {} get price {}", item.name, price);
                        match price.parse::<f32>() {
                            Ok(p) => self.persistent(item, p.round() as usize),
                            _ => println!("parse price {} err", price),
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn run(&self) {
        loop {
            match self.fetch() {
                Some(html) => {
                    self.parse(html);
                }
                _ => break,
            }
            thread::sleep(time::Duration::from_secs(3));
        }
    }
}

impl Crawl for YyypCrawler {
    fn db(&self) -> &DbHelper {
        &self.db_helper
    }

    fn build_url(&self) -> String {
        todo!()
    }

    fn parse(&self, html: String) {
        todo!()
    }

    fn run(&self) {
        todo!()
    }
}

impl Crawl for IgxeCrawler {
    fn db(&self) -> &DbHelper {
        &self.db_helper
    }

    fn build_url(&self) -> String {
        todo!()
    }

    fn parse(&self, html: String) {
        todo!()
    }

    fn run(&self) {
        todo!()
    }
}
