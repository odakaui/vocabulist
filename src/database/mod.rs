use std::error::Error;
use rusqlite::{params, Connection};
use crate::{Expression};

#[macro_use]
/// sql!(update_in_anki_for_expression, query, params=[tx: &Transaction, expression: &str, in_anki: bool])
macro_rules! sql {
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

/// Setup the database.
///
/// # Arguments
///
/// * `conn` - A &Connection object

fn initialize(conn: &Connection) -> Result<(), Box<dyn Error>> {
    query::table::create_expressions(conn)?;
    query::table::create_pos(conn)?;
    query::table::create_sentences(conn)?;
    query::table::create_surface_strings(conn)?;
    query::table::create_expressions_pos_sentences_surface_strings(conn)?;

    Ok(())
}

/// Open a connection to the database.
///
/// # Arguments
///
/// * `path` - An &str with the file system path to the database

pub fn connect(path: &str) -> Connection {
    let conn = Connection::open(path).expect("Failed to connect to the database");
    initialize(&conn).expect("Failed to initialize database");

    conn
}

/// Create a list of Expression objects with sentences that have not been inserted into the database.
///
/// # Arguments
///
/// * `conn` - A &Connection object
/// * `sentence_list` - A Vec<String> of sentences that have already been imported
/// * `expression_list` - A list of Expression objects

pub fn filter_imported_expression_list(sentence_list: &Vec<String>, expression_list: Vec<Expression>) -> Vec<Expression> {
    let mut tmp_expression_list: Vec<Expression> = Vec::new();
    for expression in expression_list.into_iter() {
        let mut is_duplicate = false;
        for sentence in sentence_list.iter() {
            let sentence_string = &expression.get_sentence()[0];
            if sentence_string == sentence {
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

/// Create a list of sentences that have already been imported and that are in sentence_list.
pub fn select_imported_sentence_list(conn: &Connection, sentence_list: &Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
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

pub fn insert_expression_list(conn: &mut Connection, expression_list: Vec<Expression>, callback: &dyn Fn()) -> Result<(), Box<dyn Error>> {
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
pub fn select_expression_list(conn: &Connection, in_anki: bool, is_excluded: bool, is_learned: bool, order_by: &str, is_asc: bool, limit: i32) -> Result<Vec<Expression>, Box<dyn Error>> {
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


pub fn update_is_excluded(conn: &mut Connection, expression_list: &Vec<Expression>, is_excluded: bool, callback: &dyn Fn()) -> Result<(), Box<dyn Error>> {
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

const UPDATE_IN_ANKI_FOR_EXPRESSION: &str = "UPDATE expressions SET in_anki = ? WHERE expression = ?;";
                                            
pub fn select_pos_for_expression(conn: &Connection, expression: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let params = params![expression];
    let mut statement = conn.prepare(SELECT_POS_FOR_EXPRESSION)?;

    let pos_list: Vec<String> = statement.query_map(params, |row| Ok(row.get(0)?))?
        .map(|x| x.unwrap())
        .collect();

    Ok(pos_list)
}

pub fn select_sentence_for_expression(conn: &Connection, expression: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let params = params![expression];
    let mut statement = conn.prepare(SELECT_SENTENCE_FOR_EXPRESSION)?;

    let sentence_list: Vec<String> = statement.query_map(params, |row| Ok(row.get(0)?))?
        .map(|x| x.unwrap())
        .collect();

    Ok(sentence_list)
}

sql!(update_in_anki_for_expression, UPDATE_IN_ANKI_FOR_EXPRESSION, params=[conn: &Connection, in_anki: u32, expression: &str]);
