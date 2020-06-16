extern crate clap;
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

    println!("{:?}", matches)
}
