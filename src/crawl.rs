use crate::db::DbHelper;
use crate::http;
use crate::item::{Item, PriceInfo};
use crate::utils;

use rand::Rng;
use serde_json::{Error as JsonError, Value};
use std::{thread, time};

#[derive(PartialEq, Eq, Hash)]
pub enum Target {
    Buff,
    Yyyp,
}

impl From<&str> for Target {
    fn from(s: &str) -> Self {
        match s {
            "buff" => Target::Buff,
            "yyyp" => Target::Yyyp,
            _ => panic!("Unknown target: {}", s),
        }
    }
}

pub trait Crawl {
    fn name(&self) -> &str;

    fn alert(&self, message: &str) {
        let message = format!(
            "[{}]      \n{}: {}",
            utils::current_time(),
            self.name(),
            message
        );
        println!("{}", message);
        utils::alert(message.as_str());
    }

    fn success(&self) {
        let message = format!(
            "[{}] {}: get json items success",
            utils::current_time(),
            self.name(),
        );
        println!("{}", message);
    }

    fn db(&self) -> &DbHelper;

    fn parse(&self, html: String) -> bool;

    fn run(&self, api: &str) {
        loop {
            match http::get(api) {
                Ok(html) => {
                    self.parse(html);
                }
                _ => self.alert("fetch api failed"),
            }
            self.sleep();
        }
    }

    fn persistent(&self, item: Item, price: &str) {
        match price.parse::<f32>() {
            Ok(p) => match self.db().get_item_id(&item) {
                None => {}
                Some(id) => {
                    self.db().add_price_info(&PriceInfo::new(
                        id,
                        utils::current_date(),
                        utils::round(p),
                    ));
                }
            },
            _ => self.alert(format!("parse price {} err", price).as_str()),
        }
    }

    fn sleep(&self) {
        thread::sleep(time::Duration::from_secs(
            rand::thread_rng().gen_range(180..240),
        ));
    }
}

pub struct BuffCrawler {
    db_helper: DbHelper,
}

pub struct YyypCrawler {
    db_helper: DbHelper,
}

impl BuffCrawler {
    pub fn new(db_helper: DbHelper) -> Self {
        Self { db_helper }
    }

    pub fn build_url() -> String {
        let mut url = String::from(utils::API_BUFF);
        url.push_str(utils::current_timestamp().as_str());
        url
    }
}

impl Crawl for BuffCrawler {
    fn name(&self) -> &str {
        "BuffCrawler"
    }

    fn db(&self) -> &DbHelper {
        &self.db_helper
    }

    fn parse(&self, html: String) -> bool {
        fn get_value(value: &Value, key: &str) -> String {
            value[key]["localized_name"]
                .as_str()
                .unwrap_or(utils::DEFAULT)
                .to_string()
        }
        let result_value: Result<Value, JsonError> = serde_json::from_str(html.as_str());
        match result_value {
            Ok(value) => {
                let data = &value["data"];
                match data["items"].as_array() {
                    Some(data_items) => {
                        self.success();
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
                                self.persistent(item, price);
                            }
                        }
                    }
                    _ => self.alert("read whole json failed, cant find items"),
                }
            }
            _ => self.alert("read whole json failed"),
        }
        true
    }
}

impl YyypCrawler {
    pub fn new(db_helper: DbHelper) -> Self {
        Self { db_helper }
    }

    fn get_type_by_name(name: &str) -> String {
        let mut vec = Vec::new();
        let name = String::from(name);
        for c in name.chars() {
            if c == '（' || c == '(' {
                break;
            }
            vec.push(c.to_string());
        }
        vec.concat()
    }
}

impl Crawl for YyypCrawler {
    fn name(&self) -> &str {
        "YyypCrawler"
    }

    fn db(&self) -> &DbHelper {
        &self.db_helper
    }

    fn parse(&self, html: String) -> bool {
        let result_value: Result<Value, JsonError> = serde_json::from_str(html.as_str());
        match result_value {
            Ok(value) => match value["Code"].as_u64() {
                Some(0) => match value["Data"].as_array() {
                    Some(datas) => {
                        self.success();
                        let _ = datas
                            .into_iter()
                            .map(|data| match data["TypeName"].as_str() {
                                Some("匕首") | Some("手套") => {
                                    let name = data["CommodityName"].as_str().unwrap();
                                    if let Some(price) = data["Price"].as_str() {
                                        self.persistent(
                                            Item::new(
                                                name.to_string(),
                                                data["TypeName"].as_str().unwrap().to_string(),
                                                YyypCrawler::get_type_by_name(name),
                                                data["Exterior"].as_str().unwrap().to_string(),
                                                data["Quality"].as_str().unwrap().to_string(),
                                                data["Rarity"].as_str().unwrap().to_string(),
                                                name.contains("StatTrak"),
                                            ),
                                            price,
                                        );
                                    }
                                }
                                _ => {}
                            })
                            .collect::<()>();
                        true
                    }
                    _ => {
                        self.alert("json datas empty");
                        false
                    }
                },
                _ => {
                    let msg = format!("http request fail, message: {:?}", value["Msg"].as_str());
                    self.alert(msg.as_str());
                    false
                }
            },
            _ => {
                self.alert("read whole json failed");
                false
            }
        }
    }
}
