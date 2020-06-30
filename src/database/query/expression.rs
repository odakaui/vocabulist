use std::error::Error;
use rusqlite::{Transaction, params};

/* Expression Functions */

pub fn insert(tx: &Transaction, string: &str) -> Result<(), Box<dyn Error>> {
    let params = params![string];
    let query = "INSERT OR IGNORE INTO expressions (expression) VALUES (?) ON CONFLICT (expression) DO UPDATE SET frequency = frequency + 1;"; 

    tx.execute(query, params)?;

    Ok(())
}

pub fn update_is_excluded(tx: &Transaction, string: &str, is_excluded: bool) -> Result<(), Box<dyn Error>> {
    let is_excluded = if is_excluded { 1 } else { 0 };
    let params = params![is_excluded, string];
    let query = "UPDATE expressions SET is_excluded = ? WHERE expression = ?;";

    tx.execute(query, params)?;

    Ok(())
}

pub fn select_id(tx: &Transaction, string: &str) -> Result<i32, Box<dyn Error>> {
    let params = params![string];
    let query = "SELECT id FROM expressions WHERE expression = ?;";

    let id: i32 = tx.query_row(query, params, |row| row.get(0))?;

    Ok(id)
}

query_function!(select_is_excluded, Transaction, "SELECT is_excluded FROM expressions WHERE expression = ?;");
