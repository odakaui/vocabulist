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

pub struct Term {
    expression: String,
    pos: String,
    sentence: String,
    surface_string: String,
}

impl Term {
    pub fn new(expression: String, pos: String, sentence: String, surface_string: String) -> Self {
        Term {
            expression,
            pos,
            sentence,
            surface_string,
        }
    }

    pub fn expression(&self) -> &str {
        &self.expression
    }

    pub fn pos(&self) -> &str {
        &self.pos
    }

    pub fn sentence(&self) -> &str {
        &self.sentence
    }

    pub fn surface_string(&self) -> &str {
        &self.surface_string
    }
}

/// DEPRACATED: get Connection object for database
pub fn connect(path: &PathBuf) -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open(path)?;

    Ok(conn)
}

/// get Connection object for database
pub fn connection(path: &PathBuf) -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open(path)?;

    Ok(conn)
}

pub fn transaction(conn: &mut Connection) -> Result<Transaction, Box<dyn Error>> {
    let tx = conn.transaction()?;

    Ok(tx)
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

/// check sentence exists in database
pub fn select_sentence_exists(conn: &Connection, sentence: &str) -> Result<bool, Box<dyn Error>> {
    let mut statement = conn.prepare("SELECT sentence FROM sentences WHERE sentence = ?;")?;

    Ok(statement.exists(params![sentence])?)
}

/// insert expression into database
pub fn insert_expression(tx: &Transaction, expression: &str) -> Result<(), Box<dyn Error>> {
    let query = "INSERT OR IGNORE INTO expressions (expression) VALUES (?) 
        ON CONFLICT (expression) DO UPDATE SET frequency = frequency + 1;";

    tx.execute(query, params![expression])?;

    Ok(())
}

/// insert pos into database
pub fn insert_pos(tx: &Transaction, pos: &str) -> Result<(), Box<dyn Error>> {
    let query = "INSERT OR IGNORE INTO pos (pos) VALUES (?);";

    tx.execute(query, params![pos])?;

    Ok(())
}

/// insert sentence into database
pub fn insert_sentence(tx: &Transaction, sentence: &str) -> Result<(), Box<dyn Error>> {
    let query = "INSERT OR IGNORE INTO sentences (sentence) VALUES (?);";

    tx.execute(query, params![sentence])?;

    Ok(())
}

/// insert surface string into database
pub fn insert_surface_string(tx: &Transaction, surface_string: &str) -> Result<(), Box<dyn Error>> {
    let query = "INSERT OR IGNORE INTO surface_strings (surface_string) VALUES (?);";

    tx.execute(query, params![surface_string])?;

    Ok(())
}

/// insert Term into database
pub fn insert_term(tx: &Transaction, term: &Term) -> Result<(), Box<dyn Error>> {
    let expression = term.expression();
    let pos = term.pos();
    let sentence = term.sentence();
    let surface_string = term.surface_string();

    insert_expression(tx, expression)?;
    insert_pos(tx, pos)?;
    insert_sentence(tx, sentence)?;
    insert_surface_string(tx, surface_string)?;

    let exp_id = select_id_for_expression(tx, expression)?;
    let pos_id = select_id_for_pos(tx, pos)?;
    let sen_id = select_id_for_sentence(tx, sentence)?;
    let sur_id = select_id_for_surface_string(tx, surface_string)?;

    insert_join(tx, exp_id, pos_id, sen_id, sur_id)?;

    Ok(())
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

/// select id for a given expression
fn select_id_for_expression(tx: &Transaction, expression: &str) -> Result<i32, Box<dyn Error>> {
    let query = "SELECT id FROM expressions WHERE expression=?;";

    let id: i32 = tx.query_row(query, params![expression], |row| row.get(0))?;

    Ok(id)
}

/// select id for a given pos
fn select_id_for_pos(tx: &Transaction, pos: &str) -> Result<i32, Box<dyn Error>> {
    let query = "SELECT id FROM pos WHERE pos=?;";

    let id: i32 = tx.query_row(query, params![pos], |row| row.get(0))?;

    Ok(id)
}

/// select id for a given sentence
fn select_id_for_sentence(tx: &Transaction, sentence: &str) -> Result<i32, Box<dyn Error>> {
    let query = "SELECT id FROM sentences WHERE sentence=?;";

    let id: i32 = tx.query_row(query, params![sentence], |row| row.get(0))?;

    Ok(id)
}

/// select id for a given surface string
fn select_id_for_surface_string(
    tx: &Transaction,
    surface_string: &str,
) -> Result<i32, Box<dyn Error>> {
    let query = "SELECT id FROM surface_strings WHERE surface_string=?;";

    let id: i32 = tx.query_row(query, params![surface_string], |row| row.get(0))?;

    Ok(id)
}

/// insert join
fn insert_join(
    tx: &Transaction,
    exp_id: i32,
    pos_id: i32,
    sen_id: i32,
    sur_id: i32,
) -> Result<(), Box<dyn Error>> {
    let query = "INSERT OR IGNORE INTO expressions_pos_sentences_surface_strings 
        (expression_id, pos_id, sentence_id, surface_string_id) VALUES (?,?,?,?);";

    tx.execute(query, params![exp_id, pos_id, sen_id, sur_id])?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::DropBehavior;

    #[test]
    #[ignore]
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
    #[ignore]
    fn test_select_sentence_list() -> Result<(), Box<dyn Error>> {
        let db_path = setup("test_select_sentence_list.db")?;

        let mut conn = connect(&db_path)?;
        initialize(&conn)?;

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
    #[ignore]
    fn test_insert_expression() -> Result<(), Box<dyn Error>> {
        let db_path = setup("test_insert_expression.db")?;
        let mut conn = connect(&db_path)?;

        initialize(&conn)?;

        let mut tx = conn.transaction()?;

        let expression_list: Vec<String> = vec![
            "プロ", "プロ", "野球", "は", "今", "客", "を", "人", "まで", "入れ", "て", "試合",
            "を", "し", "て", "い", "ます",
        ]
        .iter()
        .map(|expression| expression.to_string())
        .collect();

        // terms with frequency 2
        let frequency_list: Vec<String> = ["プロ", "て", "を"]
            .iter()
            .map(|expression| expression.to_string())
            .collect();

        // get number of non duplicate expressions
        let mut tmp_list = expression_list.clone();
        tmp_list.sort();
        tmp_list.dedup();

        let num_rows = tmp_list.len();

        // insert expressions into database
        for expression in expression_list.iter() {
            insert_expression(&tx, &expression)?;
        }

        // close transaction
        tx.set_drop_behavior(DropBehavior::Commit);
        tx.finish()?;

        // get all expressions from database
        let mut statement = conn.prepare("SELECT expression FROM expressions;")?;
        let result_list: Vec<String> = statement
            .query_map(params![], |row| Ok(row.get(0)?))?
            .map(|row| row.unwrap_or_default())
            .collect();

        // get expressoins with frequency > 1 from database
        let mut statement =
            conn.prepare("SELECT expression FROM expressions WHERE frequency > 1;")?;
        let frequency_result_list: Vec<String> = statement
            .query_map(params![], |row| Ok(row.get(0)?))?
            .map(|row| row.unwrap_or_default())
            .collect();

        tear_down(db_path)?;

        // check number of rows equals length of expression list - 1
        assert_eq!(result_list.len(), num_rows);

        // check each expression is in database
        for result in result_list.iter() {
            assert!(expression_list.contains(result));
        }

        // check each expression with frequency > 1 in database
        for result in frequency_result_list.iter() {
            assert!(frequency_list.contains(result));
        }

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_insert_sentence() -> Result<(), Box<dyn Error>> {
        // setup
        let db_path = setup("test_insert_sentence.db")?;
        let mut conn = connect(&db_path)?;

        initialize(&conn)?;

        let sentence_list: Vec<String> = vec![
            "プロ野球は今、客を５０００人まで入れて試合をしています。", 
            "プロ野球は今、客を５０００人まで入れて試合をしています。", 
            "８月からはイベントの客の数を増やしてもいいと国が言っていたため、会場の半分まで客を増やす予定でした。"
        ]
        .iter()
        .map(|sentence| sentence.to_string())
        .collect();

        let mut sorted_list: Vec<String> = sentence_list.clone();

        sorted_list.sort();
        sorted_list.dedup();

        // test
        let mut tx = conn.transaction()?;

        for sentence in sentence_list.iter() {
            insert_sentence(&tx, sentence)?;
        }

        tx.set_drop_behavior(DropBehavior::Commit);
        tx.finish()?;

        // results
        let mut statement = conn.prepare("SELECT sentence FROM sentences;")?;
        let mut result_list: Vec<String> = statement
            .query_map(params![], |row| Ok(row.get(0)?))?
            .map(|row| row.unwrap_or_default())
            .collect();

        result_list.sort();

        statement.finalize()?;

        // cleanup
        conn.close().or(Err("Failed to close database"))?;
        tear_down(db_path)?;

        // assert
        assert_eq!(result_list, sorted_list);
        assert_ne!(result_list, sentence_list);

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_insert_pos() -> Result<(), Box<dyn Error>> {
        // setup
        let db_path = setup("test_insert_pos.db")?;
        let mut conn = connect(&db_path)?;

        initialize(&conn)?;

        let pos_list: Vec<String> = vec![
            "名詞", "名詞", "助詞", "名詞", "記号", "名詞", "助詞", "名詞", "名詞", "名詞", "名詞",
            "名詞",
        ]
        .iter()
        .map(|sentence| sentence.to_string())
        .collect();

        let mut sorted_list: Vec<String> = pos_list.clone();
        sorted_list.sort();
        sorted_list.dedup();

        // test
        let mut tx = conn.transaction()?;

        for pos in pos_list.iter() {
            insert_pos(&tx, pos)?;
        }

        tx.set_drop_behavior(DropBehavior::Commit);
        tx.finish()?;

        // results
        let mut statement = conn.prepare("SELECT pos FROM pos;")?;
        let mut result_list: Vec<String> = statement
            .query_map(params![], |row| Ok(row.get(0)?))?
            .map(|row| row.unwrap_or_default())
            .collect();

        result_list.sort();

        statement.finalize()?;

        // cleanup
        conn.close().or(Err("Failed to close database"))?;
        tear_down(db_path)?;

        // assert
        assert_eq!(sorted_list, sorted_list);

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_insert_surface_string() -> Result<(), Box<dyn Error>> {
        // setup
        let db_path = setup("test_insert_surface_string.db")?;
        let mut conn = connect(&db_path)?;

        initialize(&conn)?;

        let surface_string_list: Vec<String> = vec![
            "プロ", "プロ", "野球", "は", "今", "客", "を", "人", "まで", "入れ", "て", "試合",
            "を", "し", "て", "い", "ます",
        ]
        .iter()
        .map(|surface_string| surface_string.to_string())
        .collect();

        let mut sorted_list: Vec<String> = surface_string_list.clone();

        sorted_list.sort();
        sorted_list.dedup();

        // test
        let mut tx = conn.transaction()?;

        for surface_string in surface_string_list.iter() {
            insert_surface_string(&tx, surface_string)?;
        }

        tx.set_drop_behavior(DropBehavior::Commit);
        tx.finish()?;

        // results
        let mut statement = conn.prepare("SELECT surface_string FROM surface_strings;")?;
        let mut result_list: Vec<String> = statement
            .query_map(params![], |row| Ok(row.get(0)?))?
            .map(|row| row.unwrap_or_default())
            .collect();
        result_list.sort();

        statement.finalize()?;

        // cleanup
        conn.close().or(Err("Failed to close database"))?;
        tear_down(db_path)?;

        // assert
        assert_eq!(result_list, sorted_list);
        assert_ne!(result_list, surface_string_list);

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_insert_term() -> Result<(), Box<dyn Error>> {
        // setup
        let db_path = setup("test_insert_surface_string.db")?;
        let mut conn = connect(&db_path)?;

        initialize(&conn)?;

        let expected_num_expressions = 3;
        let expected_num_pos = 2;
        let expected_num_sentences = 1;
        let expected_num_surface_strings = 3;

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
            "名前は何ですか".to_string(),
            "何".to_string(),
        ));
        term_list.push(Term::new(
            "何".to_string(),
            "名詞".to_string(),
            "名前は何ですか".to_string(),
            "何".to_string(),
        ));

        // test
        let mut tx = conn.transaction()?;

        for term in term_list.iter() {
            insert_term(&tx, term)?;
        }

        tx.set_drop_behavior(DropBehavior::Commit);
        tx.finish()?;

        // results
        let mut statement = conn.prepare("SELECT count(*) FROM expressions;")?;
        let num_expressions: i32 = statement
            .query_map(params![], |row| Ok(row.get(0)?))?
            .map(|row| row.unwrap())
            .collect::<Vec<i32>>()[0];

        statement.finalize()?;

        let mut statement = conn.prepare("SELECT count(*) FROM pos;")?;
        let num_pos: i32 = statement
            .query_map(params![], |row| Ok(row.get(0)?))?
            .map(|row| row.unwrap())
            .collect::<Vec<i32>>()[0];

        statement.finalize()?;

        let mut statement = conn.prepare("SELECT count(*) FROM sentences;")?;
        let num_sentences: i32 = statement
            .query_map(params![], |row| Ok(row.get(0)?))?
            .map(|row| row.unwrap())
            .collect::<Vec<i32>>()[0];

        statement.finalize()?;

        let mut statement = conn.prepare("SELECT count(*) FROM surface_strings;")?;
        let num_surface_strings: i32 = statement
            .query_map(params![], |row| Ok(row.get(0)?))?
            .map(|row| row.unwrap())
            .collect::<Vec<i32>>()[0];

        statement.finalize()?;

        let mut expression_statement =
            conn.prepare("SELECT expression FROM expressions WHERE expression=?;")?;

        let mut pos_statement = conn.prepare(
            "SELECT pos FROM pos 
                JOIN expressions_pos_sentences_surface_strings ON pos_id = pos.id
                JOIN expressions ON expressions.id = expression_id
                WHERE expression=?;",
        )?;

        let mut sentence_statement = conn.prepare(
            "SELECT sentence FROM sentences 
                JOIN expressions_pos_sentences_surface_strings ON sentence_id = sentences.id
                JOIN expressions ON expressions.id = expression_id
                WHERE expression=?;",
        )?;

        let mut surface_string_statement = conn.prepare("SELECT surface_string FROM surface_strings 
                JOIN expressions_pos_sentences_surface_strings ON surface_string_id = surface_strings.id
                JOIN expressions ON expressions.id = expression_id
                WHERE expression=?;",
        )?;

        let mut result_list: Vec<(String, Vec<String>)> = Vec::new();
        for term in term_list.iter() {
            let expression_list = expression_statement
                .query_map(params![term.expression()], |row| Ok(row.get(0)?))?
                .map(|row| row.unwrap_or_default())
                .collect();

            result_list.push((term.expression().to_string(), expression_list));

            let pos_list = pos_statement
                .query_map(params![term.expression()], |row| Ok(row.get(0)?))?
                .map(|row| row.unwrap_or_default())
                .collect();

            result_list.push((term.pos().to_string(), pos_list));

            let sentence_list = sentence_statement
                .query_map(params![term.expression()], |row| Ok(row.get(0)?))?
                .map(|row| row.unwrap_or_default())
                .collect();

            result_list.push((term.sentence().to_string(), sentence_list));

            let surface_string_list = surface_string_statement
                .query_map(params![term.expression()], |row| Ok(row.get(0)?))?
                .map(|row| row.unwrap_or_default())
                .collect();

            result_list.push((term.surface_string().to_string(), surface_string_list));
        }

        // cleanup
        expression_statement.finalize()?;
        pos_statement.finalize()?;
        sentence_statement.finalize()?;
        surface_string_statement.finalize()?;

        conn.close().or(Err("Failed to close database"))?;
        tear_down(db_path)?;

        // assert
        assert_eq!(
            num_expressions, expected_num_expressions,
            "unexpected number of expressions in database"
        );
        assert_eq!(
            num_pos, expected_num_pos,
            "unexpected number of pos in database"
        );
        assert_eq!(
            num_sentences, expected_num_sentences,
            "unexpected number of sentences in database"
        );
        assert_eq!(
            num_surface_strings, expected_num_surface_strings,
            "unexpected nuber of surface strings in database"
        );

        for result in result_list.iter() {
            assert!(result.1.contains(&result.0), "join table is broken");
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
