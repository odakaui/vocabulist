use std::fs;
use std::process::{Command, Stdio};
use std::io::Write;

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

    if let Some(matches) = matches.subcommand_matches("import") {
        let path =  matches.value_of("path").unwrap();
        let attr = fs::metadata(path).expect("Can't get file attribute");

        if attr.is_dir() {
            // Parse each file in the directory
        } else {
            let contents = fs::read_to_string(path).expect("Can't read file")
                .replace(&['「', '」', '『', '』', '…'][..], "")
                .replace(&['。'][..], "。\n")
                .replace(&['？'][..], "？\n")
                .replace(&['！'][..], "！\n");
            println!("{}", contents);

            let sentence_list: Vec<&str> = contents.lines().filter(|x| x != &"").map(|x| x.trim()).collect();
            println!("{:?}", sentence_list);

            for sentence in sentence_list {
                let mut jumanpp = Command::new("jumanpp")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Failed to start jumanpp process");
                
                let stdin = jumanpp.stdin.as_mut().expect("Failed to get jumanpp stdin");
                    stdin.write_all(sentence.as_bytes()).expect("Failed to write to jumanpp stdin");

                let jumanpp_output = jumanpp.wait_with_output().expect("Failed to wait for jumanpp");
                let token_string = String::from_utf8_lossy(&jumanpp_output.stdout);
                let token_list: Vec<&str> = token_string.lines().collect();

                println!("{:?}", token_list);
            }
        }



    }
}
