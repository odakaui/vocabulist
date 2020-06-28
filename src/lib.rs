mod database;
mod expression;
mod tokenizer;
mod progress_bar;

use std::fs;
use std::path::Path;

use clap::{ArgMatches};

use rusqlite::{Connection};

use expression::{Expression};

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

fn import_file(conn: &mut Connection, path: &str) {
    let sentence_list = open_file(path);
    let expression_list = tokenizer::tokenize_sentence_list(&sentence_list);
    let expression_list = database::deduplicate_expression_list(&conn, sentence_list, expression_list);    
    
    database::insert_expression_list(conn, expression_list);

}

pub fn import(p: Preference, m: &ArgMatches) {
    // Initialize the database
    let database_path = p.database_path.as_ref();
    let mut conn = database::connect(database_path);

    let path =  Path::new(m.value_of("path").unwrap());

    if path.is_dir() {
        // Parse each file in the directory
        for path in fs::read_dir(path).expect("Could not get file list") {
            if let Ok(file) = path {
                println!("Importing {}", &file.path().to_str().unwrap());
                crate::import_file(&mut conn, &file.path().to_str().unwrap());
                println!("");
            }
        }
    } else {
        if let Some(file) = path.to_str() {
            println!("Importing {}", file);
            crate::import_file(&mut conn, file);
            println!("");
        }
    }
}

pub fn list(p: Preference, m: &ArgMatches) {
    // Initialize the database
    let database_path = p.database_path.as_ref();
    let conn = database::connect(database_path);

    let in_anki     = m.is_present("anki");
    let is_excluded = m.is_present("excluded");
    let is_learned  = m.is_present("learned");
    let order_by    = match m.value_of("order") {
        Some(order) => order,
        None => "frequency"
    };
    let is_asc      = m.is_present("asc");
    let limit       = m.value_of("number").unwrap().parse::<i32>().unwrap();

    database::select_expression_list(&conn, in_anki, is_excluded, is_learned, order_by, is_asc, limit);
}

pub fn exclude(p: Preference, m: &ArgMatches) {
    // Initialize the database
    let database_path = p.database_path.as_ref();
    let mut conn = database::connect(database_path);

    if let Some(path) = m.value_of("path") {
            let file_content = fs::read_to_string(path).expect("Failed to open file");
            let line_list = file_content.split_whitespace();
            let expression_list: Vec<Expression> = line_list.map(|x| Expression::new(x.to_string())).collect();
            let len: u64 = expression_list.len() as u64;
            let pb = progress_bar::new(len);

            crate::database::update_is_excluded(&mut conn, expression_list, true, &|| pb.inc(1))
                .expect("Failed to update database");

            pb.finish_with_message("Excluded");
    }
}
