use std::collections::HashMap;
use rusqlite::{Connection, params, Result, Row};
use crate::item::{Item, ItemType, Type};
use crate::PriceInfo;

const DB_FILE: &'static str = "data.db";

fn create_db() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(DB_FILE)?;
    let _ = conn.execute("drop table Typo", []);
    let _ = conn.execute("drop table WearType", []);
    let _ = conn.execute("drop table Quality", []);
    let _ = conn.execute("drop table Rarity", []);
    let _ = conn.execute(
        "create table Typo (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT NOT NULL,
            name_zh TEXT NOT NULL
        )",
        [],
    );
    let _ = conn.execute(
        "create table WearType (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT NOT NULL,
            name_zh TEXT NOT NULL
        )",
        [],
    );
    let _ = conn.execute(
        "create table Quality (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT NOT NULL,
            name_zh TEXT NOT NULL
        )",
        [],
    );
    let _ = conn.execute(
        "create table Rarity (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT NOT NULL,
            name_zh TEXT NOT NULL
        )",
        [],
    );
    let _ = conn.execute(
        "create table Item (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            typo      INTEGER NOT NULL,
            category  TEXT NOT NULL,
            name      TEXT NOT NULL,
            ware_type INTEGER NOT NULL,
            quality   INTEGER,
            rarity    INTEGER,
            stat_trak INTEGER NOT NULL
        )",
        [],
    );
    let _ = conn.execute(
        "create table PriceInfo (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            item_id INTEGER NOT NULL,
            date    TEXT NOT NULL,
            price   REAL NOT NULL
        )",
        [],
    );

    Ok(conn)
}

#[allow(unused)]
fn init_base_data() -> Result<(), rusqlite::Error> {
    let conn = create_db()?;
    let mut stmt = conn.prepare("insert into ItemType (name, name_zh) values (?1, ?2)")?;
    stmt.execute(params!["Kinfe", "刀"]);
    stmt.execute(params!["Pistol", "手枪"]);
    stmt.execute(params!["Rifle", "步枪"]);
    stmt.execute(params!["SubmachineGun", "冲锋枪"]);
    stmt.execute(params!["Shotgun", "霰弹枪"]);
    stmt.execute(params!["MachineGun", "机枪"]);
    stmt.execute(params!["Sticker", "贴纸"]);
    stmt.execute(params!["Gloves", "手套"]);
    stmt.execute(params!["Agent", "探员"]);
    stmt.execute(params!["Other", "其他"]);

    let mut stmt = conn.prepare("insert into WearType (name, name_zh) values (?1, ?2)")?;
    stmt.execute(params!["NoWare", "无磨损"]);
    stmt.execute(params!["FactoryNew", "崭新出产"]);
    stmt.execute(params!["MinimalWare", "略有磨损"]);
    stmt.execute(params!["FieldTested", "久经沙场"]);
    stmt.execute(params!["WellWorn", "破损不堪"]);
    stmt.execute(params!["BattleScarred", "战痕累累"]);

    let mut stmt = conn.prepare("insert into Quality (name, name_zh) values (?1, ?2)")?;
    stmt.execute(params!["ConsumerGrade", "消费级"]);
    stmt.execute(params!["IndustrialGrade", "工业级"]);
    stmt.execute(params!["MilSpec", "军规级"]);
    stmt.execute(params!["Restricted", "受限"]);
    stmt.execute(params!["Classified", "保密"]);
    stmt.execute(params!["Covert", "隐秘"]);
    stmt.execute(params!["ContrabandItems", "违禁品"]);

    let mut stmt = conn.prepare("insert into Rarity (name, name_zh) values (?1, ?2)")?;
    stmt.execute(params!["Common", "普通"]);
    stmt.execute(params!["Rare", "高级"]);
    stmt.execute(params!["Legendary", "奇异"]);
    stmt.execute(params!["Mythical", "卓越"]);
    stmt.execute(params!["Ancient", "非凡"]);
    Ok(())
}

fn load_db() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(DB_FILE)?;
    Ok(conn)
}

fn select_types(table: &str) -> Result<HashMap<String, Type>, rusqlite::Error> {
    let conn = load_db()?;
    let mut stmt = conn.prepare("select * from " + table)?;
    let mut map: HashMap<String, Type> = HashMap::new();
    stmt.query_map([], |row| {
        let typo: Type = Type {
            id: row.get(0)?,
            name: row.get(1)?,
            name_zh: row.get(2)?,
        };
        map.insert(typo.name.to_lowercase(), typo);
        Ok(())
    })?;
    Ok(map)
}

struct DbHelper {
    conn: Connection,
    item_types: HashMap<String, Type>,
    ware_types: HashMap<String, Type>,
    qualities: HashMap<String, Type>,
    rarities: HashMap<String, Type>,
}

impl DbHelper {
    fn new(conn: Connection) -> DbHelper {
        DbHelper {
            conn,
            item_types: select_types("ItemType").unwrap(),
            ware_types: select_types("WearType").unwrap(),
            qualities: select_types("Quality").unwrap(),
            rarities: select_types("Rarity").unwrap(),
        }
    }

    fn default() -> DbHelper {
        DbHelper::new(load_db().unwrap())
    }

    // fn add_item(&self, item: Item) {
    //     self.conn.execute(
    //         "INSERT INTO Item (typo, name, ware_type, quality, rarity, stat_trak) VALUES(?1, ?2, ?3, ?4, ?5, ?6);",
    //         params![
    //             ItemType::from(item.typo),
    //             item.name,
    //             item.ware_type,
    //             item.quality,
    //             item.rarity,
    //             item.stat_trak
    //         ],
    //     );
    // }

    fn add_price_info(&self, price_info: PriceInfo) {
        self.conn.execute(
            "INSERT INTO PriceInfo (item_id, date, price) VALUES(?1, ?2, ?3);",
            params![
                price_info.item_id,
                price_info.date,
                price_info.price,
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::db::DbHelper;
    use crate::item::*;

    #[test]
    fn test_create_db() {
        assert!(super::create_db().is_ok());
    }

    #[test]
    fn test_init_base_data() {
        assert!(super::init_base_data().is_ok());
    }

    #[test]
    fn test_select() {
        assert!(super::select_types("ItemType").is_ok())
    }

    #[test]
    fn test_add_item() {
        // let db_helper = DbHelper::default();
        // let item = Item {
        //     id: 0,
        //     typo: ItemType::Kinfe,
        //     name: "蝴蝶刀".to_string(),
        //     ware_type: WearType::NoWare,
        //     quality: Quality::ConsumerGrade,
        //     rarity: Rarity::Common,
        //     stat_trak: false,
        // };
        // db_helper.add_item(item);
        // assert!(true)
    }
}
