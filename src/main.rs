use std::error::Error;
use clap::{Arg, App, SubCommand};

fn main() -> Result<(), Box<dyn Error>> {
    let match_list = App::new("Vocabulist")
                            .version("0.1")
                            .author("Odaka Ui <odakaui@example.com>")
                            .about("Vocabulary database for learning Japanese")
                            .arg(Arg::with_name("config")
                                    .short("c")
                                    .long("config")
                                    .value_name("PATH")
                                    .help("Sets a custom config file")
                                    .takes_value(true))
                            .subcommand(SubCommand::with_name("import")
                                        .about("import file(s)")
                                        .arg(Arg::with_name("path")
                                            .value_name("PATH")
                                            .required(true)
                                            .help("Import file/directory to database")))
                            .subcommand(SubCommand::with_name("exclude")
                                        .about("exclude expressions")
                                        .arg(Arg::with_name("path")
                                            .value_name("PATH")
                                            .required(true)
                                            .help("Path to file of words to exclude")))
                            .subcommand(SubCommand::with_name("include")
                                        .about("include expressions")
                                        .arg(Arg::with_name("path")
                                            .value_name("PATH")
                                            .required(true)
                                            .help("Path to file of words to include")))
                            .subcommand(SubCommand::with_name("generate")
                                        .about("generate flashcards")
                                        .arg(Arg::with_name("number")
                                            .value_name("NUM")
                                            .required(false)
                                            .default_value("10")
                                            .help("Number of flashcards to generate")))
                            .subcommand(SubCommand::with_name("list")
                                        .about("list vocabulary")
                                        .arg(Arg::with_name("number")
                                            .value_name("NUM")
                                            .default_value("-1")
                                            .required(false)
                                            .help("Number of entries to list"))
                                        .arg(Arg::with_name("asc")
                                            .short("a")
                                            .long("asc")
                                            .help("Sort by ascending instead of descending"))
                                        .arg(Arg::with_name("anki")
                                            .long("anki")
                                            .help("Show expressions that are already in anki"))
                                        .arg(Arg::with_name("learned")
                                            .long("learn")
                                            .help("Show expressions that have already been learned"))
                                        .arg(Arg::with_name("excluded")
                                            .long("exclude")
                                            .help("Show expressions that have been excluded"))
                                        .arg(Arg::with_name("order")
                                            .short("o")
                                            .long("order")
                                            .takes_value(true)
                                            .possible_value("frequency")
                                            .possible_value("expression")
                                            .possible_value("id")
                                            .help("Sort by ascending instead of descending")))
                            .get_matches();
    
    
    let database_path = "/tmp/vocabulist_rs.db";
    let dictionary_path = "jmdict.db";
    let p = vocabulist_rs::Preference {
        database_path: database_path.to_string(),
        dictionary_path: dictionary_path.to_string(),
        audio: true
    };

    match match_list.subcommand() {
        ("import", Some(m)) => vocabulist_rs::import(p, m),
        ("list", Some(m)) => vocabulist_rs::list(p, m),
        ("exclude", Some(m)) => vocabulist_rs::exclude(p, m),
        ("include", Some(m)) => vocabulist_rs::include(p, m),
        ("generate", Some(m)) => vocabulist_rs::generate(p, m),
        _ => Ok(()),
    }?;

    Ok(())
}
