use clap::{App, Arg, SubCommand};
use dirs;
use std::error::Error;
use std::fs;
use vocabulist_rs::config::Config;
use vocabulist_rs::VERSION;

fn main() -> Result<(), Box<dyn Error>> {
    let match_list = App::new("Vocabulist")
        .version(VERSION)
        .author("Odaka Ui <odakaui@example.com>")
        .about("Vocabulary database for learning Japanese")
        .arg(
            Arg::with_name(VERSION)
                .short("c")
                .long("config")
                .value_name("PATH")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("import").about("import file(s)").arg(
                Arg::with_name("path")
                    .value_name("PATH")
                    .required(true)
                    .help("Import file/directory to database"),
            ),
        )
        .subcommand(
            SubCommand::with_name("exclude")
                .about("exclude expressions")
                .arg(
                    Arg::with_name("path")
                        .value_name("PATH")
                        .required(true)
                        .help("Path to file of words to exclude"),
                )
                .arg(
                    Arg::with_name("pos")
                        .long("pos")
                        .help("Exclude pos and all expressions associated with them"),
                ),
        )
        .subcommand(
            SubCommand::with_name("include")
                .about("include expressions")
                .arg(
                    Arg::with_name("path")
                        .value_name("PATH")
                        .required(true)
                        .help("Path to file of words to include"),
                )
                .arg(
                    Arg::with_name("pos")
                        .long("pos")
                        .help("Exclude pos and all expressions associated with them"),
                ),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("generate flashcards")
                .arg(
                    Arg::with_name("number")
                        .value_name("NUM")
                        .required(false)
                        .default_value("10")
                        .help("Number of flashcards to generate"),
                ),
        )
        .subcommand(SubCommand::with_name("sync").about("sync database with anki"))
        .subcommand(SubCommand::with_name("config").about("generate configuration")
                .arg(
                    Arg::with_name("homebrew")
                        .long("homebrew")
                        .help("Create config file for homebrew install"),
                )
                .arg(
                    Arg::with_name("force")
                        .short("f")
                        .long("force")
                        .help("Overwrite existing config"),
                ))
        .subcommand(
            SubCommand::with_name("list")
                .about("list vocabulary")
                .arg(
                    Arg::with_name("pos")
                        .long("pos")
                        .conflicts_with_all(&["anki", "learned", "order"])
                        .help("List pos instead of vocabulary"),
                )
                .arg(
                    Arg::with_name("number")
                        .value_name("NUM")
                        .default_value("-1")
                        .required(false)
                        .help("Number of entries to list"),
                )
                .arg(
                    Arg::with_name("asc")
                        .short("a")
                        .long("asc")
                        .help("Sort by ascending instead of descending"),
                )
                .arg(
                    Arg::with_name("anki")
                        .long("anki")
                        .help("Show expressions that are already in anki"),
                )
                .arg(
                    Arg::with_name("learned")
                        .long("learn")
                        .help("Show expressions that have already been learned"),
                )
                .arg(
                    Arg::with_name("excluded")
                        .long("exclude")
                        .help("Show expressions that have been excluded"),
                )
                .arg(
                    Arg::with_name("order")
                        .short("o")
                        .long("order")
                        .takes_value(true)
                        .possible_value("frequency")
                        .possible_value("expression")
                        .possible_value("id")
                        .help("Sort by ascending instead of descending"),
                ),
        )
        .get_matches();

    // load the config file
    let config_directory;
    let config_file;
    let home_path = dirs::home_dir().expect("Failed to get home directory");

    match !cfg!(debug_assertions) {
        // path for release
        true => {
            config_directory = home_path.join(".vocabulist_rs");
            config_file = config_directory.join("config.toml");

        },
        // path for dev
        false => {
            config_directory = home_path.join(".vocabulist_rs_dev");
            config_file = config_directory.join("config.toml");

            println!("WARNING: Running in developer mode.");
            println!("WARNING: Using {} as home directory.", config_directory.to_str().unwrap());
            println!("");
        }
    }

    // check if the config file exists
    let toml;

    match config_file.is_file() {
        true => {
            toml = fs::read_to_string(config_file)?;
        },
        false => {
            toml = String::new();

            println!("ERROR: Configuration file does not exist.");
            println!("Exiting.");
            println!("");
            println!("To create a configuration file run `vocabulist_rs config`");

            panic!("");
        }
    }

    let config: Config = toml::from_str(&toml)?;

    match match_list.subcommand() {
        ("import", Some(m)) => vocabulist_rs::import(config, m),
        ("sync", Some(m)) => vocabulist_rs::sync(config, m),
        ("list", Some(m)) => vocabulist_rs::list(config, m),
        ("config", Some(m)) => vocabulist_rs::config(config, m),
        ("exclude", Some(m)) => vocabulist_rs::exclude(config, m),
        ("include", Some(m)) => vocabulist_rs::include(config, m),
        ("generate", Some(m)) => vocabulist_rs::generate(config, m),
        _ => Ok(()),
    }?;

    Ok(())
}
