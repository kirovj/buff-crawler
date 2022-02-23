use std::collections::HashMap;
use rusqlite::{Connection, params, Result};
use crate::constant::DB_FILE;
use crate::item::Item;
use crate::PriceInfo;

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
            "INSERT INTO Item (name, class, typo, ware, quality, rarity, stat_trak) VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7);",
            params![
                item.name,
                item.class,
                item.typo,
                item.ware,
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
    use rusqlite::Connection;
    use crate::constant::DB_FILE;
    use crate::db::DbHelper;
    use crate::item::*;

    fn create_db() -> Result<(), rusqlite::Error> {
        let conn = Connection::open(DB_FILE)?;
        let _ = conn.execute(
            "create table Item (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            name      TEXT NOT NULL,
            class     TEXT NOT NULL,
            typo      TEXT NOT NULL,
            ware      TEXT NOT NULL,
            quality   TEXT,
            rarity    TEXT,
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

    #[test]
    fn test_create_db() {
        assert!(create_db().is_ok());
    }

    #[test]
    fn test_add_item() {
        let db_helper = DbHelper::default();
        let item = Item {
            id: 0,
            name: "蝴蝶刀（★） | 人工染色 (崭新出厂)".to_string(),
            class: "刀".to_string(),
            typo: "蝴蝶刀".to_string(),
            ware: "久经沙场".to_string(),
            quality: "★".to_string(),
            rarity: "隐秘".to_string(),
            stat_trak: false,
        };
        db_helper.add_item(item);
        assert!(true)
    }
}
