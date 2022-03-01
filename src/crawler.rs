use crate::db::DbHelper;
use crate::http::request;
use crate::item::{Item, PriceInfo};
use crate::utils;

use chrono::Local;
use rand::Rng;
use serde_json::{Error, Value};
use std::{thread, time};

pub trait Crawl {
    fn name(&self) -> &str;

    fn alert(&self, message: &str) {
        utils::alert(format!("{}: {}", self.name(), message).as_str());
    }

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

    fn persistent(&self, item: Item, price: f32) {
        match self.db().get_item_id(&item) {
            None => {}
            Some(id) => {
                let date = Local::now().format("%Y-%m-%d").to_string();
                self.db().add_price_info(&PriceInfo::new(id, date, price));
            }
        }
    }
}

pub fn build_crawler(target: &str, db_file: &str) -> Option<Box<dyn Crawl>> {
    let db_helper = DbHelper::new(db_file);
    match target {
        "buff" => Some(Box::new(BuffCrawler { db_helper })),
        "yyyp" => Some(Box::new(YyypCrawler { db_helper })),
        "igxe" => Some(Box::new(IgxeCrawler { db_helper })),
        _ => None,
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
    fn name(&self) -> &str {
        "BuffCrawler"
    }

    fn db(&self) -> &DbHelper {
        &self.db_helper
    }

    fn build_url(&self) -> String {
        let mut url = String::from(utils::API_BUFF);
        url.push_str(Local::now().timestamp_millis().to_string().as_str());
        url
    }

    fn parse(&self, html: String) {
        fn get_value(value: &Value, key: &str) -> String {
            value[key]["localized_name"]
                .as_str()
                .unwrap_or(utils::DEFAULT)
                .to_string()
        }
        let result_value: Result<Value, Error> = serde_json::from_str(html.as_str());
        match result_value {
            Ok(value) => {
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
                                match price.parse::<f32>() {
                                    Ok(p) => {
                                        println!("process {} get price {}", item.name, p);
                                        self.persistent(item, utils::round(p));
                                    }
                                    _ => self.alert(format!("parse price {} err", price).as_str()),
                                }
                            }
                        }
                    }
                    _ => self.alert("read whole json failed, cant find items"),
                }
            }
            _ => self.alert("read whole json failed"),
        }
    }

    fn run(&self) {
        loop {
            match self.fetch() {
                Some(html) => {
                    self.parse(html);
                }
                _ => {
                    self.alert("fetch api failed");
                    break;
                }
            }
            thread::sleep(time::Duration::from_secs(
                rand::thread_rng().gen_range(15..30),
            ));
        }
    }
}

impl Crawl for YyypCrawler {
    fn name(&self) -> &str {
        "YyypCrawler"
    }

    fn db(&self) -> &DbHelper {
        &self.db_helper
    }

    fn build_url(&self) -> String {
        todo!()
    }

    fn parse(&self, _html: String) {
        todo!()
    }

    fn run(&self) {
        todo!()
    }
}

impl Crawl for IgxeCrawler {
    fn name(&self) -> &str {
        "IgxeCrawler"
    }

    fn db(&self) -> &DbHelper {
        &self.db_helper
    }

    fn build_url(&self) -> String {
        todo!()
    }

    fn parse(&self, _html: String) {
        todo!()
    }

    fn run(&self) {
        todo!()
    }
}
