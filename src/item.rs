use std::collections::HashMap;
use std::fs;
use crate::constant::BASE_DATA_FILE;

pub struct Categories {
    item_types: HashMap<String, Type>,
    ware_types: HashMap<String, Type>,
    qualities: HashMap<String, Type>,
    rarities: HashMap<String, Type>,
}

enum Category {
    Item,
    Wear,
    Quality,
    Rarity,
}

impl Categories {
    pub fn from_json(path: &str) -> Categories {
        let json = fs::read_to_string(path).unwrap();
        let value: serde_json::Value = serde_json::from_str(json.as_str()).unwrap();
        Categories {
            item_types: Categories::make_map("item_types", &value),
            ware_types: Categories::make_map("ware_types", &value),
            qualities: Categories::make_map("qualities", &value),
            rarities: Categories::make_map("rarities", &value),
        }
    }

    fn make_map(map_name: &str, value: &serde_json::Value) -> HashMap<String, Type> {
        let mut map = HashMap::new();
        let items = &value[map_name].as_array().unwrap();
        for item in items.into_iter() {
            let text = item.to_string();
            let mut splits = text.split("|");
            let name = splits.next().unwrap().to_string();
            let name_zh = splits.next().unwrap().to_string();
            map.insert(name.to_lowercase(), Type { name, name_zh });
        }
        map
    }

    pub fn get_type(&self, category: Category, key: String) -> Option<&Type> {
        match category {
            Category::Item => self.item_types.get(&key),
            Category::Wear => self.ware_types.get(&key),
            Category::Quality => self.qualities.get(&key),
            Category::Rarity => self.rarities.get(&key),
        }
    }
}

#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub name_zh: String,
}

pub struct Item {
    pub id: u32,
    pub name: String,
    pub item_type: String,
    pub ware_type: String,
    pub quality: String,
    pub rarity: String,
    pub stat_trak: bool,
}

#[derive(Debug)]
pub struct PriceInfo {
    pub id: usize,
    pub item_id: u32,
    pub date: String,
    pub price: f32,
}

impl PriceInfo {
    pub fn new(id: usize, item_id: u32, date: String, price: f32) -> Self {
        PriceInfo {
            id,
            item_id,
            date,
            price,
        }
    }
}
