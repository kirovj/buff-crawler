// 类别
pub enum ItemType {
    Kinfe,
    Pistol,
    Rifle,
    SubmachineGun,
    Shotgun,
    MachineGun,
    Sticker,
    Gloves,
    Agent,
    Other,
}

// 磨损种类
pub enum WearType {
    NoWare,
    FactoryNew,
    MinimalWare,
    FieldTested,
    WellWorn,
    BattleScarred,
}

// 品质
pub enum Quality {
    ConsumerGrade,
    IndustrialGrade,
    MilSpec,
    Restricted,
    Classified,
    Covert,
    MeleeWeapons,
    ContrabandItems,
}

// 稀有
pub enum Rarity {
    Common,
    Rare,
    Legendary,
    Mythical,
    Ancient,
}

#[derive(Debug)]
pub struct Type {
    pub id: u32,
    pub name: String,
    pub name_zh: String,
}

pub struct Item {
    pub id: u32,
    pub typo: ItemType,
    pub name: String,
    pub ware_type: WearType,
    pub quality: Quality,
    pub rarity: Rarity,
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
