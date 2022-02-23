use std::collections::HashMap;
use rusqlite::{Connection, params, Result};
use crate::constant::DB_FILE;
use crate::item::Item;
use crate::PriceInfo;

pub static DB_HELPER: DbHelper = {
    let conn = Connection::open(DB_FILE).unwrap();
    DbHelper { conn }
};

pub struct DbHelper {
    conn: Connection,
}

impl DbHelper {

    fn find_item_id(&self, item: &Item) -> Result<u32> {
        let mut stmt = self.conn.prepare(
            "SELECT id from Item \
            where name = ?1 \
            and class = ?2 \
            and typo = ?3 \
            and ware = ?4 \
            and quality = ?5 \
            and rarity = ?6 \
            and stat_trak = ?7"
        ).unwrap();
        stmt.query_row(
            params![item.name, item.class, item.typo, item.ware, item.quality, item.rarity, item.stat_trak],
            |row| row.get(0),
        )
    }

    pub fn get_item_id(&self, item: &Item) -> Option<u32> {
        match self.find_item_id(item) {
            Ok(id) => Some(id),
            _ => {
                self.add_item(item);
                match self.find_item_id(item) {
                    Ok(_id) => Some(_id),
                    _ => None
                }
            }
        }
    }

    pub fn add_item(&self, item: &Item) {
        self.conn.execute(
            "INSERT INTO Item \
            (name, class, typo, ware, quality, rarity, stat_trak) \
            VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7);",
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

    pub fn add_price_info(&self, price_info: &PriceInfo) {
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
    use crate::db::{DB_HELPER, DbHelper};
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
            quality   TEXT NOT NULL,
            rarity    TEXT NOT NULL,
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

    fn test_item() -> Item {
        Item {
            id: 0,
            name: "蝴蝶刀（★） | 人工染色 (崭新出厂)".to_string(),
            class: "刀".to_string(),
            typo: "蝴蝶刀".to_string(),
            ware: "久经沙场".to_string(),
            quality: "★".to_string(),
            rarity: "隐秘".to_string(),
            stat_trak: false,
        }
    }

    #[test]
    fn test_add_item() {
        let item = test_item();
        DB_HELPER.add_item(&item);
        assert!(true)
    }

    #[test]
    fn test_get_item_id() {
        let item = test_item();
        assert_eq!(Some(1), DB_HELPER.get_item_id(&item));
    }
}
