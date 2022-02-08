// 类别
enum Type {
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
enum WearType {
    FactoryNew,
    MinimalWare,
    FieldTested,
    WellWorn,
    BattleScarred,
}

// 品质
enum Quality {
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
enum Rarity {
    Common,
    LessCommon,
    Rare,
    Mythical,
    Legendary,
    Ancient,
    DevastatinglyRare,
    Immortal,
}

pub struct Item {
    id: u32,
    typo: Type,
    catagory: String,
    name: String,
    ware_type: WearType,
    quality: Quality,
    rarity: Rarity,
    stat_trak: bool,
}

pub struct PriceInfo {
    id: usize,
    item_id: u32,
    date: String,
    price: f32,
}
