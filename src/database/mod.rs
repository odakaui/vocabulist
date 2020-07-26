use crate::Expression;
use rusqlite::{params, Connection, Transaction};
use std::error::Error;
use std::path::PathBuf;

#[macro_use]
/// sql!(update_in_anki_for_expression, query, params=[tx: &Transaction, expression: &str, in_anki: bool])
macro_rules! sql {
    ($fn_name:ident, $query:expr, params=[$conn:ident:$conn_type:ty]) => {
        pub fn $fn_name($conn: $conn_type)  -> Result<(), Box<dyn Error>> {
            let query = $query;

            $conn.execute(query, params![])?;

            Ok(())
        }
    };
    ($fn_name:ident, $query:expr, params=[$conn:ident:$conn_type:ty, $($param:ident:$type:ty),+]) => {
        pub fn $fn_name($conn: $conn_type, $($param: $type),+)  -> Result<(), Box<dyn Error>> {
            let params = params![$($param),+];
            let query = $query;

            $conn.execute(query, params)?;

            Ok(())
        }
    }
}

mod query;

/// get Connection object for database
pub fn connect(path: &PathBuf) -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open(path)?;

    Ok(conn)
}

/// initialize database tables
pub fn initialize(conn: &Connection) -> Result<(), Box<dyn Error>> {
    query::table::create_expressions(conn)?;
    query::table::create_pos(conn)?;
    query::table::create_sentences(conn)?;
    query::table::create_surface_strings(conn)?;
    query::table::create_expressions_pos_sentences_surface_strings(conn)?;

    Ok(())
}

pub fn select_sentence_list(conn: &Connection) -> Result<Vec<String>, Box<dyn Error>> {
    let sentence_list: Vec<String> = Vec::new();

    Ok(sentence_list)
}

/// Create a list of sentences that have already been imported and that are in sentence_list.
pub fn select_imported_sentence_list(
    conn: &Connection,
    sentence_list: &Vec<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut duplicate_sentence_list: Vec<String> = Vec::new();
    for sentence in sentence_list.iter() {
        if query::sentence::exists(conn, sentence)? {
            duplicate_sentence_list.push(sentence.to_string());
        }
    }

    Ok(duplicate_sentence_list)
}

/// Insert a vector of Expression objects into the database.
///
/// # Arguments
///
/// * `conn` - A &Connection object
/// * `expression_list` - The Expression objects to add to the database
/// * `callback` - A function that is called after each expression is inserted

pub fn insert_expression_list(
    conn: &mut Connection,
    expression_list: Vec<Expression>,
    callback: &dyn Fn(),
) -> Result<(), Box<dyn Error>> {
    let tx = conn.transaction()?;

    for expression in expression_list.iter() {
        let expression_string = expression.get_expression();
        let pos_string = &expression.get_pos()[0];
        let sentence_string = &expression.get_sentence()[0];
        let surface_string = &expression.get_surface_string()[0];

        query::expression::insert(&tx, expression_string)?;
        query::pos::insert(&tx, pos_string)?;
        query::sentence::insert(&tx, sentence_string)?;
        query::surface_string::insert(&tx, surface_string)?;

        let expression_id = query::expression::select_id(&tx, expression_string)?;
        let pos_id = query::pos::select_id(&tx, pos_string)?;
        let sentence_id = query::sentence::select_id(&tx, sentence_string)?;
        let surface_string_id = query::surface_string::select_id(&tx, surface_string)?;

        query::insert_join(&tx, expression_id, pos_id, sentence_id, surface_string_id)?;

        callback();
    }

    tx.commit()?;

    Ok(())
}

fn create_select_query(
    in_anki: bool,
    is_excluded: bool,
    is_learned: bool,
    order_by: &str,
    is_asc: bool,
    max: i32,
) -> String {
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
        _ => query.push_str("frequency "),
    }

    match is_asc {
        true => query.push_str("ASC "),
        false => query.push_str("DESC "),
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
pub fn select_expression_list(
    conn: &Connection,
    in_anki: bool,
    is_excluded: bool,
    is_learned: bool,
    order_by: &str,
    is_asc: bool,
    limit: i32,
) -> Result<Vec<Expression>, Box<dyn Error>> {
    let query = create_select_query(in_anki, is_excluded, is_learned, order_by, is_asc, limit);

    let mut select_expression = conn.prepare(&query)?;

    let tmp_list = select_expression.query_map(params![], |row| {
        let expression: String = row.get(0)?;
        Ok(Expression::new(expression))
    })?;

    let mut expression_list: Vec<Expression> = Vec::new();
    for expression in tmp_list {
        if let Ok(expression) = expression {
            expression_list.push(expression);
        }
    }

    Ok(expression_list)
}

pub fn update_is_excluded_for_expression_list(
    conn: &mut Connection,
    expression_list: &Vec<Expression>,
    is_excluded: bool,
    callback: &dyn Fn(),
) -> Result<(), Box<dyn Error>> {
    let tx = conn.transaction()?;

    for expression in expression_list {
        let expression_string = expression.get_expression();
        query::expression::update_is_excluded(&tx, expression_string, is_excluded)?;
        callback();
    }

    tx.commit()?;

    Ok(())
}

const SELECT_POS_FOR_EXPRESSION: &str = "SELECT pos FROM pos JOIN expressions_pos_sentences_surface_strings ON pos_id = pos.id JOIN expressions ON expressions.id = expression_id WHERE expression = ?;";

const SELECT_SENTENCE_FOR_EXPRESSION: &str = "SELECT sentence FROM sentences JOIN expressions_pos_sentences_surface_strings ON sentence_id = sentences.id JOIN expressions ON expressions.id = expression_id WHERE expression = ?;";

const UPDATE_IN_ANKI_FOR_EXPRESSION: &str =
    "UPDATE expressions SET in_anki = ? WHERE expression = ?;";

const RESET_IN_ANKI: &str = "UPDATE expressions SET in_anki = 0 WHERE in_anki = 1;";

const UPDATE_IS_EXCLUDED_FOR_POS: &str = "UPDATE pos SET is_excluded = ?2 WHERE pos = ?1;";

const UPDATE_IS_EXCLUDED_FOR_EXPRESSION: &str =
    "UPDATE expressions SET is_excluded = ?2 WHERE expression = ?1;";

const SELECT_EXPRESSION_LIST_FOR_POS: &str = "SELECT expression FROM expressions JOIN expressions_pos_sentences_surface_strings ON expression_id = expressions.id JOIN pos ON pos.id = pos_id WHERE pos = ?;";

pub fn select_pos_for_expression(
    conn: &Connection,
    expression: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let params = params![expression];
    let mut statement = conn.prepare(SELECT_POS_FOR_EXPRESSION)?;

    let pos_list: Vec<String> = statement
        .query_map(params, |row| Ok(row.get(0)?))?
        .map(|x| x.unwrap())
        .collect();

    Ok(pos_list)
}

pub fn select_sentence_for_expression(
    conn: &Connection,
    expression: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let params = params![expression];
    let mut statement = conn.prepare(SELECT_SENTENCE_FOR_EXPRESSION)?;

    let sentence_list: Vec<String> = statement
        .query_map(params, |row| Ok(row.get(0)?))?
        .map(|x| x.unwrap())
        .collect();

    Ok(sentence_list)
}

pub fn select_pos_list(
    conn: &Connection,
    is_excluded: bool,
    is_asc: bool,
    limit: i32,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut query = "SELECT pos FROM pos ".to_string();

    if !is_excluded {
        query.push_str("WHERE is_excluded = 0 ");
    }

    match is_asc {
        true => query.push_str("ORDER BY pos ASC "),
        false => query.push_str("ORDER BY pos DESC "),
    }

    if limit > -1 {
        query.push_str(&format!("LIMIT {}", limit)[..]);
    }

    let mut statement = conn.prepare(&query)?;

    let pos_list: Vec<String> = statement
        .query_map(params![], |row| Ok(row.get(0)?))?
        .map(|x| x.unwrap())
        .collect();

    Ok(pos_list)
}

pub fn update_is_excluded_for_pos_list(
    conn: &mut Connection,
    pos_list: &Vec<String>,
    is_excluded: bool,
    callback: &dyn Fn(),
) -> Result<(), Box<dyn Error>> {
    let is_excluded = match is_excluded {
        true => 1,
        false => 0,
    };

    for pos in pos_list.iter() {
        update_is_excluded_for_pos(conn, pos, is_excluded)?;

        let expression_list = select_expression_list_for_pos(conn, pos)?;

        for expression in expression_list.iter() {
            update_is_excluded_for_expression(conn, expression, is_excluded)?;
        }

        callback();
    }

    Ok(())
}

fn select_expression_list_for_pos(
    conn: &Connection,
    pos: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut statement = conn.prepare(SELECT_EXPRESSION_LIST_FOR_POS)?;

    let expression_list: Vec<String> = statement
        .query_map(params![pos], |row| Ok(row.get(0)?))?
        .map(|x| x.unwrap())
        .collect();

    Ok(expression_list)
}

sql!(
    update_in_anki_for_expression,
    UPDATE_IN_ANKI_FOR_EXPRESSION,
    params = [conn: &Connection, in_anki: u32, expression: &str]
);

sql!(reset_in_anki, RESET_IN_ANKI, params = [conn: &Connection]);

sql!(
    update_is_excluded_for_pos,
    UPDATE_IS_EXCLUDED_FOR_POS,
    params = [conn: &Connection, pos: &str, is_excluded: i32]
);

sql!(
    update_is_excluded_for_expression,
    UPDATE_IS_EXCLUDED_FOR_EXPRESSION,
    params = [conn: &Connection, expression: &str, is_excluded: i32]
);

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_initialize() {
        let db_path = setup("test_initialize.db").expect("Failed to setup `initialize test`");

        let conn = connect(&db_path).expect("Failed to connect to database");

        initialize(&conn).expect("Failed to initialize dabatabase tables");

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
    }

    #[test]
    fn test_select_sentence_list() {}

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
}
