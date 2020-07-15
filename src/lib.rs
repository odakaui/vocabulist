mod anki;
mod database;
mod dictionary;
mod expression;
mod posconverter;
mod progress_bar;
mod tokenizer;

use clap::ArgMatches;
use expression::Expression;
use itertools::Itertools;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use tokenizer::jumanpp::Jumanpp;
use tokenizer::{Token, Tokenizer};

pub struct Preference {
    pub database_path: String,
    pub dictionary_path: String,
    pub audio: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    database_path: String,
    dictionary_path: String,
    anki: AnkiConnect,
}

#[derive(Deserialize, Serialize)]
pub struct AnkiConnect {
    deck_name: String,
    model_name: String,
    allow_duplicates: bool,
    duplicate_scope: String,
    audio: bool,
    fields: Vec<Vec<String>>,
    tags: Vec<String>,
}

impl Config {
    pub fn new(database_path: String, dictionary_path: String, anki: AnkiConnect) -> Config {
        Config {
            database_path,
            dictionary_path,
            anki,
        }
    }

    pub fn default(config_directory_path: PathBuf) -> Config {
        let deck_name = "Default".to_string();
        let model_name = "Basic".to_string();
        let allow_duplicates = false;
        let duplicate_scope = "deck".to_string();
        let audio = false;
        let fields = vec![
            vec!["Front".to_string(), "Back".to_string()],
            vec!["expression".to_string(), "definition".to_string()],
        ];
        let tags = vec!["vocabulist".to_string()];
        let anki = AnkiConnect::new(
            deck_name,
            model_name,
            allow_duplicates,
            duplicate_scope,
            audio,
            fields,
            tags,
        );

        Config {
            database_path: config_directory_path
                .join("vocabulist_rs.db")
                .to_str()
                .unwrap()
                .to_string(),
            dictionary_path: config_directory_path
                .join("jmdict.db")
                .to_str()
                .unwrap()
                .to_string(),
            anki: anki,
        }
    }

    fn database_path(&self) -> &str {
        &self.database_path
    }

    fn dictionary_path(&self) -> &str {
        &self.dictionary_path
    }

    fn anki(&self) -> &AnkiConnect {
        &self.anki
    }
}

impl AnkiConnect {
    pub fn new(
        deck_name: String,
        model_name: String,
        allow_duplicates: bool,
        duplicate_scope: String,
        audio: bool,
        fields: Vec<Vec<String>>,
        tags: Vec<String>,
    ) -> AnkiConnect {
        AnkiConnect {
            deck_name,
            model_name,
            allow_duplicates,
            duplicate_scope,
            audio,
            fields,
            tags,
        }
    }

    fn deck_name(&self) -> &str {
        &self.deck_name
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }

    fn allow_duplicates(&self) -> bool {
        self.allow_duplicates
    }

    fn duplicate_scope(&self) -> &str {
        &self.duplicate_scope
    }

    fn audio(&self) -> bool {
        self.audio
    }

    fn fields(&self) -> &Vec<Vec<String>> {
        &self.fields
    }

    fn tags(&self) -> &Vec<String> {
        &self.tags
    }
}

/// Open a file and clean the contents
fn open_file(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path)
        .expect("Can't read file")
        .replace(&['「', '」', '『', '』', '…'][..], "")
        .replace(&['。'][..], "。\n")
        .replace(&['？'][..], "？\n")
        .replace(&['！'][..], "！\n");

    let sentence_list: Vec<String> = contents
        .lines()
        .filter(|x| x != &"")
        .map(|x| x.trim().to_string())
        .collect();

    sentence_list
}

fn token_list_to_expression_list(token_list: Vec<Token>) -> Vec<Expression> {
    let mut expression_list: Vec<Expression> = Vec::new();
    for token in token_list.into_iter() {
        let expression = Expression::new(token.get_token().to_string())
            .pos(token.get_pos().clone())
            .sentence(token.get_sentence().clone())
            .surface_string(token.get_surface_string().clone());

        expression_list.push(expression);
    }

    expression_list
}

fn database_connection(database_path: &str) -> Connection {
    database::connect(database_path)
}

fn format_anki_definition(
    definition_list: &Vec<Vec<String>>,
    is_specific_definition: bool,
    is_specific_kanji: bool,
) -> String {
    let mut definition_string = String::new();

    if !is_specific_definition {
        definition_string.push_str("WARNING: Not filtered by pos.<br>\n");
    }

    if !is_specific_kanji {
        definition_string.push_str("WARNING: Not filtered by kanji. <br>\n");
    }

    definition_string.push_str("<ol>\n");
    for definition in definition_list.iter() {
        definition_string.push_str(" <li>");
        for (i, d) in definition.iter().enumerate() {
            match i {
                0 => definition_string.push_str(d),
                _ => definition_string.push_str(&format!("; {}", d)),
            };
        }
        definition_string.push_str("</li>\n");
    }
    definition_string.push_str("</ol>");

    definition_string
}

fn format_anki_reading(reading_list: &Vec<String>) -> String {
    let mut reading_string = String::new();
    for (i, reading) in reading_list.iter().enumerate() {
        match i {
            0 => reading_string.push_str(reading),
            _ => reading_string.push_str(&format!("; {}", reading)),
        };
    }

    reading_string
}

fn format_anki_sentence(sentence_list: &Vec<String>) -> String {
    sentence_list[0].to_string()
}

fn create_flashcards_from_expression_list(
    p: Config,
    conn: &mut Connection,
    dict: &Connection,
    expression_list: Vec<Expression>,
    max: i32,
    callback: &dyn Fn(),
) -> Result<(), Box<dyn Error>> {
    let mut i = 0;
    for expression in expression_list.iter() {
        let expression_string = &expression.get_expression();

        let (definition_list, is_specific_kanji) =
            dictionary::select_definition_for_expression(&dict, expression_string)?;
        let pos_list = database::select_pos_for_expression(&conn, expression_string)?;
        let reading_list = dictionary::select_reading_for_expression(&dict, expression_string)?;
        let sentence_list = database::select_sentence_for_expression(&conn, expression_string)?;

        if definition_list.len() == 0 {
            database::update_is_excluded_for_expression_list(
                conn,
                &vec![expression.clone()],
                true,
                &|| {},
            )?;
            continue;
        }

        let (definition_list, is_specific_definition) = dictionary::filter_definition_with_pos_list(
            &definition_list,
            &posconverter::convert_pos_list(&pos_list),
        );

        // remove duplicate entries
        let definition_list = definition_list.into_iter().unique().collect();

        let definition_string =
            format_anki_definition(&definition_list, is_specific_definition, is_specific_kanji);
        let expression_string = expression.get_expression();
        let reading_string = format_anki_reading(&reading_list);
        let sentence_string = format_anki_sentence(&sentence_list);
        let url_list = anki::create_url_list(expression_string, &reading_list);

        anki::insert_note(
            &p,
            &definition_string,
            &expression_string,
            &reading_string,
            &sentence_string,
            &url_list,
        )?;
        database::update_in_anki_for_expression(conn, 1u32, expression_string)?;

        i += 1;

        if i >= max {
            break;
        }

        callback();
    }

    Ok(())
}

pub fn import(p: Config, m: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // Initialize the database
    let database_path = p.database_path();
    let mut conn = database::connect(database_path);

    let path = Path::new(m.value_of("path").unwrap());

    fn import_file(conn: &mut Connection, path: &str) {
        let sentence_list = open_file(path);

        let len = sentence_list.len() as u64;
        let pb = progress_bar::new(len, "Tokenizing");
        let jumanpp = Jumanpp::new();
        let expression_list = token_list_to_expression_list(
            jumanpp.tokenize_sentence_list(&sentence_list, &|| pb.inc(1)),
        );
        pb.finish_with_message("Tokenized");

        let duplicate_sentence_list = database::select_imported_sentence_list(conn, &sentence_list)
            .expect("Failed to retrieve sentences from the database");
        let expression_list =
            database::filter_imported_expression_list(&duplicate_sentence_list, expression_list);

        let len = expression_list.len() as u64;
        let pb = progress_bar::new(len, "Importing");
        database::insert_expression_list(conn, expression_list, &|| pb.inc(1))
            .expect("Failed to insert expression");

        pb.finish_with_message("Imported");
    }

    if path.is_dir() {
        // Parse each file in the directory
        for path in fs::read_dir(path).expect("Could not get file list") {
            if let Ok(file) = path {
                println!("Importing {}", &file.path().to_str().unwrap());
                import_file(&mut conn, &file.path().to_str().unwrap());
                println!("");
            }
        }
    } else {
        if let Some(file) = path.to_str() {
            println!("Importing {}", file);
            import_file(&mut conn, file);
            println!("");
        }
    }

    Ok(())
}

pub fn list(p: Config, m: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // Initialize the database
    let database_path = p.database_path();
    let conn = database::connect(database_path);

    match m.is_present("pos") {
        true => {
            let is_excluded = m.is_present("excluded");
            let is_asc = m.is_present("asc");
            let limit = m.value_of("number").unwrap().parse::<i32>().unwrap();

            let pos_list = database::select_pos_list(&conn, is_excluded, is_asc, limit)?;

            for pos in pos_list {
                println!("{}", pos);
            }
        }
        false => {
            let in_anki = m.is_present("anki");
            let is_excluded = m.is_present("excluded");
            let is_learned = m.is_present("learned");
            let order_by = match m.value_of("order") {
                Some(order) => order,
                None => "frequency",
            };
            let is_asc = m.is_present("asc");
            let limit = m.value_of("number").unwrap().parse::<i32>().unwrap();

            let expression_list = database::select_expression_list(
                &conn,
                in_anki,
                is_excluded,
                is_learned,
                order_by,
                is_asc,
                limit,
            )
            .expect("Failed to get expressions from database");

            for expression in expression_list {
                println!("{}", expression.get_expression());
            }
        }
    }

    Ok(())
}

pub fn exclude(p: Config, m: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // Initialize the database
    let database_path = p.database_path();
    let mut conn = database::connect(database_path);

    if let Some(path) = m.value_of("path") {
        let file_content = fs::read_to_string(path).expect("Failed to open file");
        let line_list = file_content.split_whitespace();

        match m.is_present("pos") {
            true => {
                let pos_list: Vec<String> = line_list.map(|x| x.to_string()).collect();
                let len: u64 = pos_list.len() as u64;
                let pb = progress_bar::new(len, "Excluding");
                crate::database::update_is_excluded_for_pos_list(
                    &mut conn,
                    &pos_list,
                    true,
                    &|| pb.inc(1),
                )?;
                pb.finish_with_message("Excluded");
            }
            false => {
                let expression_list: Vec<Expression> =
                    line_list.map(|x| Expression::new(x.to_string())).collect();
                let len: u64 = expression_list.len() as u64;
                let pb = progress_bar::new(len, "Excluding");
                crate::database::update_is_excluded_for_expression_list(
                    &mut conn,
                    &expression_list,
                    true,
                    &|| pb.inc(1),
                )?;
                pb.finish_with_message("Excluded");
            }
        }
    }

    Ok(())
}

pub fn include(p: Config, m: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // Initialize the database
    let database_path = p.database_path();
    let mut conn = database::connect(database_path);

    if let Some(path) = m.value_of("path") {
        let file_content = fs::read_to_string(path).expect("Failed to open file");
        let line_list = file_content.split_whitespace();

        match m.is_present("pos") {
            true => {
                let pos_list: Vec<String> = line_list.map(|x| x.to_string()).collect();
                let len: u64 = pos_list.len() as u64;
                let pb = progress_bar::new(len, "Including");
                crate::database::update_is_excluded_for_pos_list(
                    &mut conn,
                    &pos_list,
                    false,
                    &|| pb.inc(1),
                )?;
                pb.finish_with_message("Included");
            }
            false => {
                let expression_list: Vec<Expression> =
                    line_list.map(|x| Expression::new(x.to_string())).collect();
                let len: u64 = expression_list.len() as u64;
                let pb = progress_bar::new(len, "Including");
                crate::database::update_is_excluded_for_expression_list(
                    &mut conn,
                    &expression_list,
                    false,
                    &|| pb.inc(1),
                )?;
                pb.finish_with_message("Included");
            }
        }
    }

    Ok(())
}

pub fn generate(p: Config, m: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // Initialize the database
    let database_path = p.database_path();
    let mut conn = database::connect(database_path);

    let dictionary_path = p.dictionary_path();
    let dict = dictionary::connect(dictionary_path)?;

    if let Some(max) = m.value_of("number") {
        let max = max.parse::<i32>().unwrap();
        let limit = max * 2;

        let expression_list = database::select_expression_list(
            &conn,
            false,
            false,
            false,
            "frequency",
            false,
            limit,
        )?;

        let pb = progress_bar::new(max as u64, "Generating");

        create_flashcards_from_expression_list(p, &mut conn, &dict, expression_list, max, &|| {
            pb.inc(1)
        })?;

        pb.finish_with_message("Finished");
    }

    Ok(())
}

pub fn sync(p: Config, _: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // Initialize the database
    let conn = database_connection(p.database_path());
    let expression_list = anki::expression_list(&p)?;

    database::reset_in_anki(&conn)?;

    for expression in expression_list.iter() {
        println!("{}", expression);
        database::update_in_anki_for_expression(&conn, 1, expression)?;
    }

    Ok(())
}
