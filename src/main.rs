use std::fs;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Vocabulist")
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
                                            .help("Imports file(s) to database")))
                            .get_matches();
    
    
    let database_path = "/tmp/vocabulist_rs.db";
    vocabulist_rs::initialize_database(database_path);

    if let Some(matches) = matches.subcommand_matches("import") {
        let path =  matches.value_of("path").unwrap();
        let attr = fs::metadata(path).expect("Can't get file attribute");

        if attr.is_dir() {
            // Parse each file in the directory
        } else {
            vocabulist_rs::import_file(database_path, path);
        }
    }
}
