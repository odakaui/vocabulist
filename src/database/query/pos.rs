use rusqlite::{params, Transaction};
use std::error::Error;

/* Pos Functions */

pub fn insert(tx: &Transaction, string: &str) -> Result<(), Box<dyn Error>> {
    let params = params![string];
    let query = "INSERT OR IGNORE INTO pos (pos) VALUES (?);";
    tx.execute(query, params)?;

    Ok(())
}

pub fn select_id(tx: &Transaction, string: &str) -> Result<i32, Box<dyn Error>> {
    let params = params![string];
    let query = "SELECT id FROM pos WHERE pos = ?;";

    let id: i32 = tx.query_row(query, params, |row| row.get(0))?;

    Ok(id)
}
