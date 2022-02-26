use crate::constant::{API, API_OPEN, DEFAULT};
use crate::db::DbHelper;
use crate::http::request;
use crate::item::{Item, PriceInfo};

use chrono::Local;
use rand::Rng;
use regex::Regex;
use serde_json::Value;
use std::error::Error;
use std::fmt::Result;
use std::{thread, time};

trait Crawl {
    fn db(&self) -> DbHelper;

    fn build_url(&self) -> String;

    fn fetch(&self) -> Result<String, reqwest::Error> {
        request(self.build_url().as_str())
    }

    fn parse(&self, html: String) -> Result<(Item, usize), Box<dyn Error>>;

    fn run(&self);

    fn persistent(&self, item: Item, price: usize) -> Result<(), Box<dyn Error>> {
        match self.db().get_item_id(&item) {
            None => {}
            Some(id) => {
                let date = Local::now().format("%Y-%m-%d").to_string();
                self.db().add_price_info(&PriceInfo::new(id, date, price));
            }
        };
        Ok(())
    }
}

enum Target {
    Buff,
    Yyyp,
    Igxe,
}

pub fn build_crawler(target: Target, db_file: &str) -> dyn Crawl {
    let db_helper = DbHelper::new(db_file);
    match target {
        Target::Buff => BuffCrawler { db_helper },
        Target::Yyyp => YyypCrawler { db_helper },
        Target::Igxe => IgxeCrawler { db_helper },
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
    fn db(&self) -> DbHelper {
        self.db_helper
    }
}

impl Crawl for YyypCrawler {
    fn db(&self) -> DbHelper {
        self.db_helper
    }
}

impl Crawl for IgxeCrawler {
    fn db(&self) -> DbHelper {
        self.db_helper
    }
}

pub struct Crawler {
    db_helper: DbHelper,
}

#[allow(unused)]
impl Crawler {
    pub fn new(db_file: &str) -> Crawler {
        let db_helper = DbHelper::new(db_file);
        Crawler { db_helper }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let url_entrance = "https://buff.163.com/market/csgo";

        let html = request(url_entrance)?;
        let re = Regex::new("<li value=\"([^\"]+)\">([^<]*)</li>")?;
        let items = re
            .captures_iter(html.as_str())
            .filter_map(|cap| match (cap.get(1), cap.get(2)) {
                (Some(name), Some(name_zh)) => Some((name.as_str(), name_zh.as_str())),
                _ => None,
            })
            .collect::<Vec<(&str, &str)>>();

        for (name, name_zh) in items {
            if !name.to_string().starts_with("weapon") {
                continue;
            }
            println!("start crawl {}  |  {}", name, name_zh);
            let mut page: u8 = 1;
            loop {
                match request(self.build_url(name, page).as_str()) {
                    Ok(r) => match &serde_json::from_str(r.as_str()) {
                        Ok(v) => {
                            if page > self.process(v) {
                                break;
                            }
                        }
                        _ => {
                            println!("read json failed!");
                            break;
                        }
                    },
                    _ => {
                        println!("request failed!");
                        break;
                    }
                };
                thread::sleep(time::Duration::from_secs(
                    rand::thread_rng().gen_range(5..10),
                ));
                page += 1;
            }
        }
        Ok(())
    }

    pub fn run_without_login(&self) {
        loop {
            let mut url = String::from(API_OPEN);
            url.push_str(Local::now().timestamp_millis().to_string().as_str());
            match request(url.as_str()) {
                Ok(r) => match &serde_json::from_str(r.as_str()) {
                    Ok(v) => self.process(v),
                    _ => {
                        println!("read json failed!\n{}", r);
                        break;
                    }
                },
                _ => {
                    println!("request failed!");
                    break;
                }
            };
            thread::sleep(time::Duration::from_secs(2));
        }
    }

    fn process(&self, value: &Value) -> u8 {
        let data = &value["data"];
        let total_page = match &data["total_page"].as_u64() {
            Some(p) => {
                match &data["items"].as_array() {
                    Some(items) => {
                        for item in *items {
                            self.process_item(item);
                        }
                    }
                    _ => {}
                }
                *p as u8
            }
            _ => 0,
        };
        total_page
    }

    fn get_value(&self, value: &Value, key: &str) -> String {
        value[key]["localized_name"]
            .as_str()
            .unwrap_or(DEFAULT)
            .to_string()
    }

    fn process_item(&self, value: &Value) {
        let name = &value["short_name"].as_str().unwrap();
        let info = &value["goods_info"]["info"]["tags"];
        let ware = self.get_value(info, "exterior");
        let quality = self.get_value(info, "quality");
        let rarity = self.get_value(info, "rarity");
        let class = self.get_value(info, "type");
        let typo = self.get_value(info, "weapon");
        let stat_trak = quality.contains("StatTrak");
        print!("process item {}: ", name);
        let item = Item::new(
            name.to_string(),
            class,
            typo,
            ware,
            quality,
            rarity,
            stat_trak,
        );
        match self.db_helper.get_item_id(&item) {
            None => {}
            Some(id) => {
                let date = Local::now().format("%Y-%m-%d").to_string();
                let price = &value["sell_min_price"].as_str().unwrap();
                println!("get price {} at {}", price, date);
                match price.parse::<f32>() {
                    Ok(p) => {
                        self.db_helper
                            .add_price_info(&PriceInfo::new(id, date, p.round() as usize))
                    }
                    _ => println!("parse price {} err", price),
                }
            }
        };
    }

    fn build_url(&self, category: &str, page: u8) -> String {
        let mut api = String::from(API);
        api.push_str("&page_num=");
        api.push_str(page.to_string().as_str());
        api.push_str("&category=");
        api.push_str(category);
        println!("build url {}", api);
        api
    }
}
