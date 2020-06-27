mod database;
mod data_types;
mod tokenizer;

use std::fs;

use rusqlite::{Connection};

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
