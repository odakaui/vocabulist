use std::io::Write;
use std::process::{Command, Stdio};
use super::{Tokenizer, Token};

fn jumanpp(sentence: &str) -> String {
    let mut jumanpp = Command::new("jumanpp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start jumanpp process");

    let stdin = jumanpp.stdin.as_mut().expect("Failed to get jumanpp stdin");
    stdin
        .write_all(sentence.as_bytes())
        .expect("Failed to write to jumanpp stdin");

    let jumanpp_output = jumanpp
        .wait_with_output()
        .expect("Failed to wait for jumanpp");

    let token_string = String::from_utf8_lossy(&jumanpp_output.stdout).to_string();

    token_string
}

fn tokenize_output(sentence: &str, output: &str) -> Vec<Token> {
    let output_list: Vec<Vec<&str>> = output
        .lines()
        .map(|x| x.split(' ').collect())
        .collect();

    let mut token_list: Vec<Token> = Vec::new();
    for token in output_list.iter() {
        if token[0] != "EOS" && token[0] != "@" && token[3] != "特殊" {
            let surface_string = vec![token[0].to_string()];
            let pos = vec![token[3].to_string()];
            let sentence = vec![sentence.to_string()];
            let token = Token::new(token[2].to_string())
                .pos(pos)
                .sentence(sentence)
                .surface_string(surface_string);

            token_list.push(token);
        }
    }

    token_list
}

fn tokenize_sentence(sentence: &str) -> Vec<Token> {
    let token_string = jumanpp(sentence);
    let token_list = tokenize_output(sentence, token_string.as_ref());

    token_list
}

pub struct Jumanpp;

impl Tokenizer for Jumanpp {
    fn new() -> Self {
        Jumanpp
    }


    fn tokenize_sentence_list(&self, sentence_list: &Vec<String>, callback: &dyn Fn()) -> Vec<Token> {
        let mut token_list: Vec<Token> = Vec::new();
        for sentence in sentence_list.iter() {
            let e = tokenize_sentence(sentence);
            token_list.extend(e);

            callback();
        }

        token_list
    }

}
