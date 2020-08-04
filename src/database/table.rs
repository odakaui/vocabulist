use rusqlite::{params, Transaction};
use std::error::Error;

/// initialize database tables
pub fn initialize(tx: &Transaction) -> Result<(), Box<dyn Error>> {
    create_expressions(tx)?;
    create_pos(tx)?;
    create_sentences(tx)?;
    create_surface_strings(tx)?;
    create_join(tx)?;

    Ok(())
}

fn create_expressions(tx: &Transaction) -> Result<(), Box<dyn Error>> {
    tx.execute(
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

fn create_sentences(tx: &Transaction) -> Result<(), Box<dyn Error>> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS sentences (
                id INTEGER PRIMARY KEY, 
                sentence TEXT NOT NULL UNIQUE
                );",
        params![],
    )?;

    Ok(())
}

fn create_pos(tx: &Transaction) -> Result<(), Box<dyn Error>> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS pos (
                id INTEGER PRIMARY KEY, 
                pos TEXT NOT NULL UNIQUE, 
                is_excluded INTEGER NOT NULL DEFAULT 0
                );",
        params![],
    )?;

    Ok(())
}

fn create_surface_strings(tx: &Transaction) -> Result<(), Box<dyn Error>> {
    tx.execute(
        "CREATE TABLE IF NOT EXISTS surface_strings (
                id INTEGER PRIMARY KEY, 
                surface_string TEXT NOT NULL UNIQUE
                );",
        params![],
    )?;

    Ok(())
}

fn create_join(tx: &Transaction) -> Result<(), Box<dyn Error>> {
    tx.execute(
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

#[cfg(test)]
mod tests {
    use super::super::{connection, transaction};
    use super::*;
    use rusqlite::{Connection, Transaction};
    use std::path::PathBuf;

    #[test]
    #[ignore]
    fn test_initialize() -> Result<(), Box<dyn Error>> {
        let db_path = setup("test_initialize.db")?;

        let mut conn = connection(&db_path)?;
        let tx = transaction(&mut conn)?;
        initialize(&tx)?;
        tx.commit()?;

        let expressions_exists = table_exists(&conn, "expressions");
        let pos_exists = table_exists(&conn, "pos");
        let sentences_exists = table_exists(&conn, "sentences");
        let surface_strings_exists = table_exists(&conn, "surface_strings");
        let join_exists = table_exists(&conn, "expressions_pos_sentences_surface_strings");

        tear_down(db_path).expect("Failed to tear down `initialize test`");

        assert!(expressions_exists, "expression table doesn't exist");
        assert!(pos_exists, "pos table doesn't exist");
        assert!(sentences_exists, "sentence table doesn't exist");
        assert!(surface_strings_exists, "surface string table doesn't exist");
        assert!(join_exists, "join table doesn't exist");

        Ok(())
    }

    fn table_exists(conn: &Connection, name: &str) -> bool {
        let mut statement = conn
            .prepare("SELECT name FROM sqlite_master WHERE type=\"table\" AND name=?;")
            .unwrap();

        let table_list: Vec<String> = statement
            .query_map(params![name], |row| Ok(row.get(0).unwrap()))
            .unwrap()
            .map(|row| row.unwrap())
            .collect();

        if table_list.len() > 0 {
            true
        } else {
            false
        }
    }

    fn setup(db_name: &str) -> Result<PathBuf, Box<dyn Error>> {
        // get path to executable
        let test_dir = std::env::current_exe()?
            .parent()
            .unwrap()
            .join("test_database");

        // create tmp directory
        if !test_dir.is_dir() {
            std::fs::create_dir(&test_dir)?;
        }

        let db_path = test_dir.join(db_name);

        Ok(db_path)
    }

    fn tear_down(db_path: PathBuf) -> Result<(), Box<dyn Error>> {
        std::fs::remove_file(db_path)?;

        Ok(())
    }
}
