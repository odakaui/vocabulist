use std::error::Error;
use rusqlite::{Transaction, params};

pub mod expression;
pub mod pos;
pub mod sentence;
pub mod surface_string;
pub mod table;


pub fn insert_join(tx: &Transaction, expression_id: i32, pos_id: i32, sentence_id: i32, surface_string_id: i32) -> Result<(), Box<dyn Error>> {
    let params = params![expression_id, pos_id, sentence_id, surface_string_id];
    let query = "INSERT OR IGNORE INTO expressions_pos_sentences_surface_strings (expression_id, pos_id, sentence_id, surface_string_id) VALUES (?, ?, ?, ?);";

    tx.execute(query, params)?;

    Ok(())
}
