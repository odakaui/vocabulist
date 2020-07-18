use rusqlite::{params, Transaction};
use std::error::Error;

/* Surface String Functions */

pub fn insert(tx: &Transaction, string: &str) -> Result<(), Box<dyn Error>> {
    let params = params![string];
    let query = "INSERT OR IGNORE INTO surface_strings (surface_string) VALUES (?);";
    tx.execute(query, params)?;

    Ok(())
}

pub fn select_id(tx: &Transaction, string: &str) -> Result<i32, Box<dyn Error>> {
    let params = params![string];
    let query = "SELECT id FROM surface_strings WHERE surface_string = ?;";
    let id: i32 = tx.query_row(query, params, |row| row.get(0))?;

    Ok(id)
}
