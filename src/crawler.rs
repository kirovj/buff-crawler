use crate::db::DbHelper;
use crate::http;
use crate::item::{Item, PriceInfo};
use crate::utils;

use rand::Rng;
use serde_json::{Error, Value};
use std::collections::HashMap;
use std::{thread, time};

pub trait Crawl {
    fn name(&self) -> &str;

    fn alert(&self, message: &str) {
        println!("{}", message);
        utils::alert(format!("[{}] {}: {}", utils::current_time(), self.name(), message).as_str());
    }

    fn db(&self) -> &DbHelper;

    fn parse(&self, html: String) -> bool;

    fn run(&self);

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
            rand::thread_rng().gen_range(15..30),
        ));
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

impl BuffCrawler {
    fn build_url() -> String {
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
        let result_value: Result<Value, Error> = serde_json::from_str(html.as_str());
        match result_value {
            Ok(value) => {
                let data = &value["data"];
                match data["items"].as_array() {
                    Some(data_items) => {
                        println!("[{}] get json items success", utils::current_time());
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

    fn run(&self) {
        loop {
            match http::get(BuffCrawler::build_url().as_str()) {
                Ok(html) => {
                    self.parse(html);
                }
                _ => {
                    self.alert("fetch api failed");
                    break;
                }
            }
            self.sleep();
        }
    }
}

impl YyypCrawler {
    fn get_item_types(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let html = http::get(utils::API_YYYP_WEAPON)?;
        let value: Value = serde_json::from_str(html.as_str())?;
        if let Some(datas) = value["Data"].as_array() {
            let mut item_types = Vec::new();
            for data in datas {
                match data["Name"].as_str() {
                    Some("匕首") | Some("手套") => {
                        if let Some(children) = data["Children"].as_array() {
                            let _ = children
                                .iter()
                                .map(|child| match child["Name"].as_str() {
                                    Some("不限") | None => {}
                                    Some(_) => {
                                        if let Some(hash_name) = child["HashName"].as_str() {
                                            item_types.push(hash_name.to_string());
                                        }
                                    }
                                })
                                .collect::<()>();
                        }
                    }
                    _ => continue,
                }
            }
            if item_types.len() > 0 {
                return Ok(item_types);
            }
        }
        let message = "get item types failed";
        self.alert(message);
        panic!("{}", message);
    }

    fn run_single(&self, typo: String) {
        for i in 1.. {
            let mut map = HashMap::new();
            map.insert("gameId", "730");
            map.insert("listSortType", "1");
            map.insert("listType", "10");
            map.insert("pageSize", "20");
            map.insert("sortType", "0");
            map.insert("weapon", typo.as_str());
            let page = i.to_string();
            map.insert("pageIndex", page.as_str());
            match http::post_json(utils::API_YYYP_PAGE, &map) {
                Ok(html) => {
                    println!(
                        "[{}] start parse {} page {}",
                        utils::current_time(),
                        typo,
                        i
                    );
                    if !self.parse(html) {
                        break;
                    }
                    self.sleep();
                }
                Err(e) => self.alert(e.to_string().as_str()),
            }
        }
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
        let result_value: Result<Value, Error> = serde_json::from_str(html.as_str());
        match result_value {
            Ok(value) => match value["TotalCount"].as_u64() {
                None => {
                    self.alert("get total count failed");
                    false
                }
                Some(0) => {
                    println!("all pages processed");
                    false
                }
                Some(_) => match value["Data"].as_array() {
                    Some(datas) => {
                        let _ = datas
                            .into_iter()
                            .map(|data| {
                                if let Some(price) = data["Price"].as_str() {
                                    let name = data["CommodityName"].as_str().unwrap();
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
                            })
                            .collect::<()>();
                        true
                    }
                    _ => {
                        self.alert("get datas failed");
                        false
                    }
                },
            },
            _ => {
                self.alert("read whole json failed");
                false
            }
        }
    }

    fn run(&self) {
        match self.get_item_types() {
            Ok(types) => types
                .into_iter()
                .map(|typo| self.run_single(typo))
                .collect(),
            _ => self.sleep(),
        }
    }
}

impl Crawl for IgxeCrawler {
    fn name(&self) -> &str {
        "IgxeCrawler"
    }

    fn db(&self) -> &DbHelper {
        &self.db_helper
    }

    fn parse(&self, _html: String) -> bool {
        todo!()
    }

    fn run(&self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::YyypCrawler;

    #[test]
    fn test_get_type_from_name() {
        assert_eq!(
            "鲍伊猎刀",
            YyypCrawler::get_type_by_name("鲍伊猎刀（★ StatTrak™） | 虎牙 (崭新出厂)")
        );
    }
}
