use std::process::{Command, Stdio};
use std::io::Write;

use indicatif::{ProgressBar, ProgressStyle};

use crate::{Expression};

/// Tokenize the sentences
fn tokenize_sentence(sentence: &str) -> Vec<Expression> {
    let mut jumanpp = Command::new("jumanpp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start jumanpp process");
    
    let stdin = jumanpp.stdin.as_mut().expect("Failed to get jumanpp stdin");
        stdin.write_all(sentence.as_bytes()).expect("Failed to write to jumanpp stdin");

    let jumanpp_output = jumanpp.wait_with_output().expect("Failed to wait for jumanpp");
    let token_string = String::from_utf8_lossy(&jumanpp_output.stdout);
    let token_list: Vec<Vec<&str>> = token_string
        .lines()
        .map(|x| x.split(' ').collect())
        .collect();

    let mut expression_list: Vec<Expression> = Vec::new();
    for token in token_list {
        if token[0] != "EOS" && token[0] != "@" && token[3] != "特殊" {
            let surface_string = vec![token[0].to_string()];
            let pos = vec![token[3].to_string()];
            let sentence = vec![sentence.to_string()];
            let expression = Expression::new(token[2].to_string())
                .pos(pos)
                .sentence(sentence)
                .surface_string(surface_string);

            expression_list.push(expression);
        }
    }
    
    expression_list
}

/// Wrapper for Tokenize sentences
pub fn tokenize_sentence_list(sentence_list: &Vec<String>) -> Vec<Expression> {
    let pb = ProgressBar::new(sentence_list.len() as u64);
    pb.set_message("Tokenizing");
    pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.black} [{bar:40.black/black}] [{pos:>7}/{len:7}] {msg}")
            .progress_chars("##-"));

    let mut expression_list: Vec<Expression> = Vec::new();
    for sentence in pb.wrap_iter(sentence_list.iter()) {
        let e = tokenize_sentence(sentence);
        expression_list.extend(e);
    }

    pb.finish_with_message("Tokenized");

    expression_list
}
