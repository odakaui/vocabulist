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

mod insert;
mod query;
mod table;
mod term;

pub use table::initialize;
pub use term::Term;

pub use insert::insert_term;

/// get Connection object for database
pub fn connection(path: &PathBuf) -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open(path)?;

    Ok(conn)
}

/// get Transaction from Connection
pub fn transaction(conn: &mut Connection) -> Result<Transaction, Box<dyn Error>> {
    let tx = conn.transaction()?;

    Ok(tx)
}

/// check sentence exists in database
pub fn select_sentence_exists(conn: &Connection, sentence: &str) -> Result<bool, Box<dyn Error>> {
    let mut statement = conn.prepare("SELECT sentence FROM sentences WHERE sentence = ?;")?;

    Ok(statement.exists(params![sentence])?)
}

//select all terms from database
pub fn select_term(tx: &Transaction) -> Result<Vec<Term>, Box<dyn Error>> {
    let query = "SELECT expression, pos, sentence, surface_string FROM expressions
        JOIN expressions_pos_sentences_surface_strings ON expression_id = expressions.id
        JOIN pos ON pos.id = pos_id
        JOIN sentences ON sentences.id = sentence_id
        JOIN surface_strings ON surface_strings.id = surface_string_id;";

    let term_list = term_list(tx, query)?;

    Ok(term_list)
}

pub fn select_excluded_term(tx: &Transaction) -> Result<Vec<Term>, Box<dyn Error>> {
    let query = "SELECT expression, pos, sentence, surface_string FROM expressions
        JOIN expressions_pos_sentences_surface_strings ON expression_id = expressions.id
        JOIN pos ON pos.id = pos_id
        JOIN sentences ON sentences.id = sentence_id
        JOIN surface_strings ON surface_strings.id = surface_string_id
        WHERE expressions.is_excluded = 1;";

    let term_list: Vec<Term> = term_list(tx, query)?;

    Ok(term_list)
}

fn term_list(tx: &Transaction, query: &str) -> Result<Vec<Term>, Box<dyn Error>> {
    let mut statement = tx.prepare(query)?;
    let term_list: Vec<Term> = statement
        .query_map(params![], |row| {
            Ok(Term::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?
        .filter_map(|term| term.ok())
        .collect();

    Ok(term_list)
}

/// Insert a vector of Expression objects into the database.
///
/// # Arguments
///
/// * `conn` - A &Connection object
/// * `expression_list` - The Expression objects to add to the database
/// * `callback` - A function that is called after each expression is inserted

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
    use rusqlite::DropBehavior;

    #[test]
    fn test_select_sentence_list() -> Result<(), Box<dyn Error>> {
        let db_path = setup("test_select_sentence_list.db")?;

        let mut conn = connection(&db_path)?;

        let tx = transaction(&mut conn)?;
        initialize(&tx)?;
        tx.commit()?;

        let sentence_list: Vec<String> = vec![
            "プロ野球は今、客を５０００人まで入れて試合をしています。", 
            "８月からはイベントの客の数を増やしてもいいと国が言っていたため、会場の半分まで客を増やす予定でした。"
        ]
        .iter()
        .map(|sentence| sentence.to_string())
        .collect();

        let query = "INSERT INTO sentences (sentence) VALUES (?);";
        for sentence in sentence_list.iter() {
            conn.execute(query, params![sentence])?;
        }

        let does_exist = select_sentence_exists(&conn, &sentence_list[0])?;
        let does_not_exist = select_sentence_exists(&conn, "Hello World")?;

        tear_down(db_path)?;

        assert!(does_exist == true, "sentence does not exist when it should");
        assert!(
            does_not_exist == false,
            "sentence exists when it should not"
        );

        Ok(())
    }

    #[test]
    fn test_select_term() -> Result<(), Box<dyn Error>> {
        // setup
        let db_path = setup("test_select_term.db")?;
        let mut conn = connection(&db_path)?;

        let tx = transaction(&mut conn)?;
        initialize(&tx)?;

        let mut term_list: Vec<Term> = Vec::new();
        term_list.push(Term::new(
            "名前".to_string(),
            "名詞".to_string(),
            "名前は何ですか".to_string(),
            "名前".to_string(),
        ));
        term_list.push(Term::new(
            "は".to_string(),
            "助詞".to_string(),
            "名前は何ですか".to_string(),
            "は".to_string(),
        ));
        term_list.push(Term::new(
            "何".to_string(),
            "名詞".to_string(),
            "今のアナウンスは何だったのですか。".to_string(),
            "何".to_string(),
        ));
        term_list.push(Term::new(
            "何".to_string(),
            "名詞".to_string(),
            "名前は何ですか".to_string(),
            "何".to_string(),
        ));

        term_list.sort();

        for term in term_list.iter() {
            insert_term(&tx, term)?;
        }

        // result
        let mut results = select_term(&tx)?;
        results.sort();

        // cleanup
        tx.finish()?;
        conn.close().or(Err("Failed to close database"))?;
        tear_down(db_path)?;

        // assert
        assert_eq!(results, term_list);

        Ok(())
    }

    #[test]
    fn test_select_excluded_term() -> Result<(), Box<dyn Error>> {
        // setup
        let db_path = setup("test_select_excluded_term.db")?;
        let mut conn = connection(&db_path)?;

        let tx = transaction(&mut conn)?;
        initialize(&tx)?;

        let mut term_list: Vec<Term> = Vec::new();
        term_list.push(Term::new(
            "名前".to_string(),
            "名詞".to_string(),
            "名前は何ですか".to_string(),
            "名前".to_string(),
        ));
        term_list.push(Term::new(
            "は".to_string(),
            "助詞".to_string(),
            "名前は何ですか".to_string(),
            "は".to_string(),
        ));
        term_list.push(Term::new(
            "何".to_string(),
            "名詞".to_string(),
            "今のアナウンスは何だったのですか。".to_string(),
            "何".to_string(),
        ));
        term_list.push(Term::new(
            "何".to_string(),
            "名詞".to_string(),
            "名前は何ですか".to_string(),
            "何".to_string(),
        ));

        let mut expected_list = term_list[2..].to_vec();

        let excluded_expression = "何";

        term_list.sort();
        expected_list.sort();

        for term in term_list.iter() {
            insert_term(&tx, term)?;
        }

        let query = "UPDATE expressions SET is_excluded = 1 WHERE expression = ?;";

        tx.execute(query, params![excluded_expression])?;

        // result
        let mut results = select_excluded_term(&tx)?;
        results.sort();

        // cleanup
        tx.finish()?;
        conn.close().or(Err("Failed to close database"))?;
        tear_down(db_path)?;

        // assert
        assert_eq!(results, expected_list);
        assert_ne!(results, term_list);
        
        for term in results.iter() {
            assert_eq!(term.expression(), excluded_expression);
        }

        Ok(())
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
