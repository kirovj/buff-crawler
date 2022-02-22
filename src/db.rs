use rusqlite::{Connection, params, Result};
use crate::item;
use crate::item::Typo;

fn create_db() -> Result<Connection, rusqlite::Error> {
    let database = "data.db";
    let conn = Connection::open(database)?;
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
    Ok(conn)
}

#[allow(unused)]
fn init_base_data() -> Result<(), rusqlite::Error> {
    let conn = create_db()?;
    let mut stmt = conn.prepare("insert into Typo (name, name_zh) values (?1, ?2)")?;
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
    let database = "data.db";
    let conn = Connection::open(database)?;
    Ok(conn)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_db() {
        assert!(super::create_db().is_ok());
    }

    #[test]
    fn test_insert() {
        assert!(super::init_base_data().is_ok());
    }
}
