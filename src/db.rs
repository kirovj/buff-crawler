use rusqlite::{Connection, Result};

fn create_db() -> Result<Connection, rusqlite::Error> {
    let database = "data.db";
    let conn = Connection::open(database)?;

    let _ = conn.execute("drop table typo", []);

    conn.execute(
        "create table typo (
            id  INTEGER PRIMARY KEY;
            name    TEXT NOT NULL;
        )",
        [],
    );

    Ok(conn)
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
    fn test_load_db() {
        panic!("Make this test fail");
    }
}
