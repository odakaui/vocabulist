use std::error::Error;
use rusqlite::{Transaction, Connection, params};

/* Sentence Functions */ 

pub fn insert(tx: &Transaction, string: &str) -> Result<(), Box<dyn Error>> {
    let params = params![string];
    let query = "INSERT OR IGNORE INTO sentences (sentence) VALUES (?);";

    tx.execute(query, params)?;

    Ok(())
}

pub fn select_id(tx: &Transaction, string: &str) -> Result<i32, Box<dyn Error>> {
    let params = params![string];
    let query = "SELECT id FROM sentences WHERE sentence = ?;";

    let id: i32 = tx.query_row(query, params, |row| row.get(0))?;

    Ok(id)
}

pub fn exists(conn: &Connection, string: &str) -> Result<bool, Box<dyn Error>> {
    let params = params![string];       
    let mut query = conn.prepare("SELECT id FROM sentences WHERE sentence = ?;")?;
    let exists = query.exists(params)?;

    Ok(exists)
}
