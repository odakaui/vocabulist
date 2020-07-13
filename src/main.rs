use clap::{App, Arg, SubCommand};
use dirs;
use std::error::Error;
use std::fs;
use vocabulist_rs::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let match_list = App::new("Vocabulist")
        .version("0.1")
        .author("Odaka Ui <odakaui@example.com>")
        .about("Vocabulary database for learning Japanese")
        .arg(
            Arg::with_name("config")
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
    let home_path;
    let config_directory_path;
    let config_path;

    match !cfg!(debug_assertions) {
        true => {
            home_path = dirs::home_dir().expect("Failed to get home directory");
            config_directory_path = home_path.join(".vocabulist_rs");
            config_path = config_directory_path.join("config.toml");

            if !config_directory_path.is_dir() {
                fs::create_dir_all(&config_directory_path)?;
            }
        }
        false => {
            home_path = std::env::current_exe()?.parent().unwrap().to_path_buf();
            config_directory_path = home_path.clone();
            config_path = config_directory_path.join("config.toml");

            println!("WARNINGWARNINGWARNINGWARNINGWARNING");
            println!("WARNINGWARNINGWARNINGWARNINGWARNING");
            println!("Using target/deb as home directory.");
            println!("WARNINGWARNINGWARNINGWARNINGWARNING");
            println!("WARNINGWARNINGWARNINGWARNINGWARNING");
        }
    }

    let toml_string: String;
    match config_path.is_file() {
        true => {
            toml_string = fs::read_to_string(config_path)?;
        }
        false => {
            let config = Config::default(config_directory_path);
            toml_string = toml::to_string(&config).unwrap();

            fs::write(config_path, &toml_string)?;
        }
    }

    let config: Config = toml::from_str(&toml_string)?;

    match match_list.subcommand() {
        ("import", Some(m)) => vocabulist_rs::import(config, m),
        ("sync", Some(m)) => vocabulist_rs::sync(config, m),
        ("list", Some(m)) => vocabulist_rs::list(config, m),
        ("exclude", Some(m)) => vocabulist_rs::exclude(config, m),
        ("include", Some(m)) => vocabulist_rs::include(config, m),
        ("generate", Some(m)) => vocabulist_rs::generate(config, m),
        _ => Ok(()),
    }?;

    Ok(())
}
