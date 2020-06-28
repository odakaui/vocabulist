mod database;
mod data_types;
mod tokenizer;

use std::fs;
use std::path::Path;

use clap::{ArgMatches};

use rusqlite::{Connection};

use data_types::{Expression, SurfaceString, Pos, Sentence};

pub struct Preference {
    pub database_path: String,
}

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

pub fn import(p: Preference, m: &ArgMatches) {
    let path =  Path::new(m.value_of("path").unwrap());
    let database_path = p.database_path;

    if path.is_dir() {
        // Parse each file in the directory
        for path in fs::read_dir(path).expect("Could not get file list") {
            if let Ok(file) = path {
                println!("Importing {}", &file.path().to_str().unwrap());
                crate::import_file(&database_path, &file.path().to_str().unwrap());
                println!("");
            }
        }
    } else {
        if let Some(file) = path.to_str() {
            println!("Importing {}", file);
            crate::import_file(&database_path, file);
            println!("");
        }
    }
}

pub fn list(db: &str, in_anki: bool, is_excluded: bool, is_learned: bool, order_by: &str, is_asc: bool, limit: i32) {
    let conn = Connection::open(db).expect("Cannot open a connection to the database");
    database::select_expression_list(&conn, in_anki, is_excluded, is_learned, order_by, is_asc, limit);
}
