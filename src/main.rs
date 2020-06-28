use std::path::Path;
use std::fs;

use clap::{Arg, App, SubCommand};

fn main() {
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
                            .subcommand(SubCommand::with_name("list")
                                        .about("list vocabulary"))
                            .get_matches();
    
    
    let database_path = "/tmp/vocabulist_rs.db";
    vocabulist_rs::initialize_database(database_path);

    match match_list.subcommand() {
        ("import", Some(sub_match_list)) => {
            let path =  Path::new(sub_match_list.value_of("path").unwrap());

            if path.is_dir() {
                // Parse each file in the directory
                for path in fs::read_dir(path).expect("Could not get file list") {
                    if let Ok(file) = path {
                        println!("Importing {}", &file.path().to_str().unwrap());
                        vocabulist_rs::import_file(database_path, &file.path().to_str().unwrap());
                        println!("");
                    }
                }
            } else {
                if let Some(file) = path.to_str() {
                    println!("Importing {}", file);
                    vocabulist_rs::import_file(database_path, file);
                    println!("");
                }
            }
        },
        ("list", _) => {
            println!("Listing Vocabulary");
            vocabulist_rs::list(database_path);
            println!("");
        },
        _ => {},
    }
}
