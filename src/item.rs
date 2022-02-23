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

impl Item {
    pub fn new(name: String,
               class: String,
               typo: String,
               ware: String,
               quality: String,
               rarity: String,
               stat_trak: bool, ) -> Item {
        Item {
            id: 0,
            name,
            class,
            typo,
            ware,
            quality,
            rarity,
            stat_trak,
        }
    }
}

#[derive(Debug)]
pub struct PriceInfo {
    pub id: usize,
    pub item_id: u32,
    pub date: String,
    pub price: usize,
}

impl PriceInfo {
    pub fn new(item_id: u32, date: String, price: usize) -> Self {
        PriceInfo {
            id: 0,
            item_id,
            date,
            price,
        }
    }
}
