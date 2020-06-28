mod database;
mod data_types;
mod tokenizer;

use std::fs;

use rusqlite::{Connection, params};

use data_types::{Expression, SurfaceString, Pos, Sentence};


/// Open a file and clean the contents
fn open_file(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Can't read file")
        .replace(&['「', '」', '『', '』', '…'][..], "")
        .replace(&['。'][..], "。\n")
        .replace(&['？'][..], "？\n")
        .replace(&['！'][..], "！\n");

    let sentence_list: Vec<String> = contents.lines().filter(|x| x != &"").map(|x| x.trim().to_string()).collect();

    sentence_list
}

pub fn initialize_database(db: &str) {
    database::initialize_database(db);
}

// Duplicate data check
pub fn import_file(db: &str, path: &str) {
    let mut conn = Connection::open(db).expect("Cannot open a connection to the database");

    let sentence_list = open_file(path);
    let expression_list = tokenizer::tokenize_sentence_list(&sentence_list);
    let expression_list = database::deduplicate_expression_list(&conn, sentence_list, expression_list);    
    
    database::insert_expression_list(&mut conn, expression_list);

}

pub fn list(db: &str, anki: bool, excluded: bool, learned: bool, order: &str, asc: bool, max: i32) {
    let conn = Connection::open(db).expect("Cannot open a connection to the database");

    let mut query = "SELECT expression FROM expressions ".to_string();

    if !(anki && excluded && learned) {
        query.push_str("WHERE ");

        if !anki {
            query.push_str("in_anki = 0 ");

            if !excluded || !learned {
                query.push_str("AND ");
            }
        }
         
        if !excluded {
            query.push_str("is_excluded = 0 ");

            if !learned {
                query.push_str("AND ");
            }
        }

        if !learned {
            query.push_str("is_learned = 0 ");
        }
    }

    match order {
        "id" => query.push_str("id "),
        "expression" => query.push_str("expression "),
        _ => query.push_str("frequency ")
    }

    match asc {
        true => query.push_str("ASC "),
        false => query.push_str("DESC ")
    }

    if max > -1 {
        query.push_str(&format!("LIMIT {}", max));
    }

    println!("{}", query);

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
