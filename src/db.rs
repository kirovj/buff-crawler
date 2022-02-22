use std::collections::HashMap;
use std::fs;
use rusqlite::{Connection, params, Result, Row};
use crate::constant::DB_FILE;
use crate::item::{Item, Type};
use crate::PriceInfo;

fn create_db() -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DB_FILE)?;
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
    Ok(())
}

fn load_db() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(DB_FILE)?;
    Ok(conn)
}

struct DbHelper {
    conn: Connection,
}

impl DbHelper {
    fn new(conn: Connection) -> DbHelper {
        DbHelper { conn }
    }

    fn default() -> DbHelper {
        DbHelper::new(load_db().unwrap())
    }

    fn add_item(&self, item: Item) {
        self.conn.execute(
            "INSERT INTO Item (name, item_type, ware_type, quality, rarity, stat_trak) VALUES(?1, ?2, ?3, ?4, ?5, ?6);",
            params![
                item.name,
                item.item_type,
                item.ware_type,
                item.quality,
                item.rarity,
                item.stat_trak
            ],
        );
    }

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
    fn test_add_item() {
        let db_helper = DbHelper::default();
        let item = Item {
            id: 0,
            item_type: "knife".to_string(),
            name: "蝴蝶刀".to_string(),
            ware_type: "久经沙场".to_string(),
            quality: "★".to_string(),
            rarity: "隐秘".to_string(),
            stat_trak: false,
        };
        db_helper.add_item(item);
        assert!(true)
    }
}
