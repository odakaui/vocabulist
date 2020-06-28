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
    let p = vocabulist_rs::Preference { database_path: database_path.to_string() };

    vocabulist_rs::initialize_database(&p.database_path);

    match match_list.subcommand() {
        ("import", Some(m)) => vocabulist_rs::import(p, m),
        ("list", Some(sub_match_list)) => {
            println!("Listing Vocabulary");
            let max = sub_match_list.value_of("number").unwrap().parse::<i32>().unwrap();
            let is_asc = sub_match_list.is_present("asc");
            let order_by = match sub_match_list.value_of("order") {
                Some(order) => order,
                None => "frequency"
            };
            let is_excluded = sub_match_list.is_present("excluded");
            let in_anki = sub_match_list.is_present("anki");
            let is_learned = sub_match_list.is_present("learned");

            vocabulist_rs::list(database_path, in_anki, is_excluded, is_learned, order_by, is_asc, max);
            println!("");
        },
        _ => {},
    }
}
