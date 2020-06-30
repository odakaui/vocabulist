use std::error::Error;
use rusqlite::{Connection, params};

pub fn create_expressions(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
            "CREATE TABLE IF NOT EXISTS expressions (
                id INTEGER PRIMARY KEY,
                expression TEXT NOT NULL UNIQUE,
                frequency DEFAULT 1,
                is_excluded INTEGER DEFAULT 0,
                in_anki INTEGER NOT NULL DEFAULT 0,
                is_learned INTEGER NOT NULL DEFAULT 0
                );",
            params![],
            )?;

    Ok(())
}

pub fn create_sentences(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
            "CREATE TABLE IF NOT EXISTS sentences (
                id INTEGER PRIMARY KEY, 
                sentence TEXT NOT NULL UNIQUE
                );",
            params![],
            )?;

    Ok(())
}

pub fn create_pos(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
            "CREATE TABLE IF NOT EXISTS pos (
                id INTEGER PRIMARY KEY, 
                pos TEXT NOT NULL UNIQUE, 
                is_excluded INTEGER NOT NULL DEFAULT 0
                );",
            params![],
            )?;

    Ok(())
}

pub fn create_surface_strings(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
            "CREATE TABLE IF NOT EXISTS surface_strings (
                id INTEGER PRIMARY KEY, 
                surface_string TEXT NOT NULL UNIQUE
                );",
            params![],
            )?;

    Ok(())
}

pub fn create_expressions_pos_sentences_surface_strings(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
            "CREATE TABLE IF NOT EXISTS expressions_pos_sentences_surface_strings (
                pos_id INTEGER, 
                sentence_id INTEGER, 
                expression_id INTEGER, 
                surface_string_id INTEGER, 
                PRIMARY KEY (pos_id, sentence_id, expression_id, surface_string_id), 
                    FOREIGN KEY (sentence_id) 
                        REFERENCES sentences (id) 
                            ON DELETE CASCADE
                            ON UPDATE NO ACTION,
                    FOREIGN KEY (expression_id)
                        REFERENCES expressions (id)
                            ON DELETE CASCADE
                            ON UPDATE NO ACTION,
                    FOREIGN KEY (pos_id)
                        REFERENCES pos (id)
                            ON DELETE CASCADE
                            ON UPDATE NO ACTION,
                    FOREIGN KEY (surface_string_id)
                        REFERENCES surface_strings (id)
                            ON DELETE CASCADE
                            ON UPDATE NO ACTION
                );",
            params![],
            )?;

            Ok(())
}
