use rusqlite::{params, Connection};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{Expression};

/// Initialize the database
///
/// path: the path to the database
///
///
///
///
pub fn initialize_database(path: &str) {
    let conn = Connection::open(path).expect("Cannot open a connection to the database");

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
            ).expect("Cannot create the 'expressions' table");

    conn.execute(
            "CREATE TABLE IF NOT EXISTS sentences (
                id INTEGER PRIMARY KEY, 
                sentence TEXT NOT NULL UNIQUE
                );",
            params![],
            ).expect("Cannot create the 'sentences' table");

    conn.execute(
            "CREATE TABLE IF NOT EXISTS pos (
                id INTEGER PRIMARY KEY, 
                pos TEXT NOT NULL UNIQUE, 
                is_excluded INTEGER NOT NULL DEFAULT 0
                );",
            params![],
            ).expect("Cannot create the 'pos' table.");

    conn.execute(
            "CREATE TABLE IF NOT EXISTS surface_strings (
                id INTEGER PRIMARY KEY, 
                surface_string TEXT NOT NULL UNIQUE
                );",
            params![],
            ).expect("Cannot create the 'surface_strings' table");

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
            ).expect("Cannot create the 'pos_sentences_sstrings_words' table");
}

/// Take a list of expressions and a list of sentences.
/// Return a vector of expressions that don't have a sentence in the sentence list.
///
///
///
///
///
///
///
///
pub fn deduplicate_expression_list(conn: &Connection, sentence_list: Vec<String>, expression_list: Vec<Expression>) -> Vec<Expression> {
    let mut duplicate_sentence_list: Vec<String> = Vec::new();
    for sentence in sentence_list.iter() {
        let mut select_sentence = conn.prepare(
                "SELECT id FROM sentences WHERE sentence = ?;"
                ).expect("Unable to prepare select_sentence");

        if select_sentence.exists(params![sentence]).expect("Unable to check if sentence exists") {
            duplicate_sentence_list.push(sentence.to_string());
        
        }
    }

    let mut tmp_expression_list: Vec<Expression> = Vec::new();
    for expression in expression_list.into_iter() {
        let mut is_duplicate = false;
        for duplicate_sentence in duplicate_sentence_list.iter() {
            let sentence = &expression.get_sentence().as_ref().expect("sentence is required").0[0];
            if sentence == duplicate_sentence {
                is_duplicate = true;
                break
            }
        }

        if !is_duplicate {
            tmp_expression_list.push(expression);
        }
    }

    tmp_expression_list
}

/// Insert an expression into the database
///
///
///
///
///
///
///
pub fn insert_expression_list(conn: &mut Connection, expression_list: Vec<Expression>) {
    let pb = ProgressBar::new(expression_list.len() as u64);
    pb.set_message("Importing");
    pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.black} [{bar:40.black/black}] [{pos:>7}/{len:7}] {msg}")
            .progress_chars("##-"));

    let tx = conn.transaction().expect("Unable to create transaction");

    for expression in pb.wrap_iter(expression_list.iter()) {
        let e: &str = expression.get_expression();
        let pos = &expression.get_pos().as_ref().expect("pos is required").0[0];
        let surface_string = &expression.get_surface_string().as_ref().expect("surface string is required").0[0];
        let sentence = &expression.get_sentence().as_ref().expect("sentence is required").0[0];

        tx.execute("INSERT OR IGNORE INTO expressions (expression) VALUES (?)
                    ON CONFLICT (expression) DO UPDATE SET frequency = frequency + 1;", 
                params![e])
            .expect(&format!("Unable to insert {:?} into database", e));
        
        tx.execute("INSERT OR IGNORE INTO pos (pos) VALUES (?);",
                params![pos])
            .expect(&format!("Unable to insert {:?} into database", pos));

        tx.execute("INSERT OR IGNORE INTO surface_strings (surface_string) VALUES (?);", 
                params![surface_string])
            .expect(&format!("Unable to insert {:?} into database", surface_string));

        tx.execute("INSERT OR IGNORE INTO sentences (sentence) VALUES (?);", 
                params![sentence])
            .expect(&format!("Unable to insert {:?} into database", sentence));

        let expression_id: i32 = tx.query_row("SELECT id FROM expressions WHERE expression = ?;",
                params![e],
                |row| row.get(0))
            .expect("Unable to get expression id");

        let pos_id: i32 = tx.query_row("SELECT id FROM pos WHERE pos = ?;",
                params![pos],
                |row| row.get(0))
            .expect("Unable to get pos id");

        let surface_string_id: i32 = tx.query_row("SELECT id FROM surface_strings WHERE surface_string = ?;",
                params![surface_string],
                |row| row.get(0))
            .expect("Unable to get surface_string id");

        let sentence_id: i32 = tx.query_row("SELECT id FROM sentences WHERE sentence = ?;",
                params![sentence],
                |row| row.get(0))
            .expect("Unable to get sentence id");

        tx.execute(
                "INSERT OR IGNORE INTO expressions_pos_sentences_surface_strings (
                    expression_id,
                    pos_id,
                    sentence_id,
                    surface_string_id
                    ) VALUES (?, ?, ?, ?);",
                params![expression_id, pos_id, sentence_id, surface_string_id])
            .expect("Unable to create join table row");
    }

    tx.commit().expect("Unable to commit transaction");

    pb.finish_with_message("Imported");
}

fn create_select_query(in_anki: bool, is_excluded: bool, is_learned: bool, order_by: &str, is_asc: bool, max: i32) -> String {
    let mut query = "SELECT expression FROM expressions ".to_string();

    if !(in_anki && is_excluded && is_learned) {
        query.push_str("WHERE ");

        if !in_anki {
            query.push_str("in_anki = 0 ");

            if !is_excluded || !is_learned {
                query.push_str("AND ");
            }
        }
         
        if !is_excluded {
            query.push_str("is_excluded = 0 ");

            if !is_learned {
                query.push_str("AND ");
            }
        }

        if !is_learned {
            query.push_str("is_learned = 0 ");
        }
    }

    query.push_str("ORDER BY ");

    match order_by {
        "id" => query.push_str("id "),
        "expression" => query.push_str("expression "),
        _ => query.push_str("frequency ")
    }

    match is_asc {
        true => query.push_str("ASC "),
        false => query.push_str("DESC ")
    }

    if max > -1 {
        query.push_str(&format!("LIMIT {}", max));
    }

    query
}

/// Get a list of expressions for the given parameters
///
///
///
///
pub fn select_expression_list(conn: &Connection, in_anki: bool, is_excluded: bool, is_learned: bool, order_by: &str, is_asc: bool, limit: i32) {
    let query = create_select_query(in_anki, is_excluded, is_learned, order_by, is_asc, limit);

    let mut select_expression = conn.prepare(&query)
        .expect("Unable to prepare select");

    let expression_list = select_expression.query_map(params![], |row| {
            let expression: String = row.get(0)?;
            Ok(Expression::new(expression, None, None, None, None, None))
            }).unwrap();

    for expression in expression_list {
        if let Ok(expression) = expression {
            println!("{}", expression.get_expression());
        }
    }
}

