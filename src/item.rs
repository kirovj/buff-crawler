use std::collections::HashMap;
use std::fs;

pub struct Category {
    classes: HashMap<String, Type>,
    wares: HashMap<String, Type>,
    qualities: HashMap<String, Type>,
    rarities: HashMap<String, Type>,
}

enum CategoryType {
    Class,
    Wear,
    Quality,
    Rarity,
}

impl CategoryType {
    fn value(&self) -> &str {
        match self {
            CategoryType::Class => "Class",
            CategoryType::Wear => "Wear",
            CategoryType::Quality => "Quality",
            CategoryType::Rarity => "Rarity",
        }
    }
}

impl Category {
    pub fn from_json(path: &str) -> Category {
        let json = fs::read_to_string(path).unwrap();
        let value: serde_json::Value = serde_json::from_str(json.as_str()).unwrap();
        Category {
            classes: Category::make_map(CategoryType::Class, &value),
            wares: Category::make_map(CategoryType::Wear, &value),
            qualities: Category::make_map(CategoryType::Quality, &value),
            rarities: Category::make_map(CategoryType::Rarity, &value),
        }
    }

    fn make_map(category_type: CategoryType, value: &serde_json::Value) -> HashMap<String, Type> {
        let mut map = HashMap::new();
        let items = &value[category_type.value()].as_array().unwrap();
        for item in items.into_iter() {
            let text = item.to_string();
            let mut splits = text.split("|");
            let name = splits.next().unwrap().to_string();
            let name_zh = splits.next().unwrap().to_string();
            map.insert(name.to_lowercase(), Type { name, name_zh });
        }
        map
    }

    pub fn get_type(&self, category_type: CategoryType, key: String) -> Option<&Type> {
        match category_type {
            CategoryType::Class => self.classes.get(&key),
            CategoryType::Wear => self.wares.get(&key),
            CategoryType::Quality => self.qualities.get(&key),
            CategoryType::Rarity => self.rarities.get(&key),
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
    pub class: String,
    pub typo: String,
    pub ware: String,
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
