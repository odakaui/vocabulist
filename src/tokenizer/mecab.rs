use super::{Token, Tokenize};
use regex::Regex;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct Mecab {
    path: PathBuf,
}

impl Mecab {
    fn new(path: PathBuf) -> Self {
        Mecab { path }
    }
}

impl Tokenize for Mecab {
    fn tokenize(&self, sentence: &str) -> Result<Vec<Token>, Box<dyn Error>> {
        let output = tokenize_sentence(&self.path, sentence)?;
        let token_list = output_to_token_list(output, sentence);

        Ok(token_list)
    }
}

fn tokenize_sentence(path: &PathBuf, sentence: &str) -> Result<String, Box<dyn Error>> {
    // spawn the mecab process
    let mut mecab = Command::new(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // pipe the sentence into the process
    mecab
        .stdin
        .as_mut()
        .ok_or("Failed to open mecab stdio")?
        .write_all(sentence.as_bytes())?;

    // get the output from the process
    let output = mecab.wait_with_output()?;
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    Ok(output)
}

fn output_to_token_list(output_string: String, sentence: &str) -> Vec<Token> {
    let re = Regex::new(r"[,\t]").unwrap();

    let mut token_list: Vec<Token> = Vec::new();
    for line in output_string.lines() {
        let output: Vec<&str> = re.split(line).collect();

        if output.len() == 10 && output[1] != "記号" {
            let expression = output[7].to_string();
            let pos = vec![output[1].to_string()];
            let sentence = vec![sentence.to_string()];
            let surface_string = vec![output[0].to_string()];

            let token = Token::new(expression)
                .pos(pos)
                .sentence(sentence)
                .surface_string(surface_string);

            token_list.push(token);
        }
    }

    token_list
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sentence() -> String {
        "名前は何ですか".to_string()
    }

    fn expected_token_list() -> Vec<Token> {
        let mut token_list: Vec<Token> = Vec::new();
        token_list.push(
            Token::new("名前".to_string())
                .pos(vec!["名詞".to_string()])
                .sentence(vec!["名前は何ですか".to_string()])
                .surface_string(vec!["名前".to_string()]),
        );
        token_list.push(
            Token::new("は".to_string())
                .pos(vec!["助詞".to_string()])
                .sentence(vec!["名前は何ですか".to_string()])
                .surface_string(vec!["は".to_string()]),
        );
        token_list.push(
            Token::new("何".to_string())
                .pos(vec!["名詞".to_string()])
                .sentence(vec!["名前は何ですか".to_string()])
                .surface_string(vec!["何".to_string()]),
        );
        token_list.push(
            Token::new("です".to_string())
                .pos(vec!["助動詞".to_string()])
                .sentence(vec!["名前は何ですか".to_string()])
                .surface_string(vec!["です".to_string()]),
        );
        token_list.push(
            Token::new("か".to_string())
                .pos(vec!["助詞".to_string()])
                .sentence(vec!["名前は何ですか".to_string()])
                .surface_string(vec!["か".to_string()]),
        );

        token_list
    }

    /// test the tokenize function for the mecab backend
    /// requires:
    ///     mecab, mecab-ipadic-utf8 on ubuntu
    ///     mecab, mecab-ipadic on mac 
    #[test]
    #[ignore]
    fn test_mecab_tokenize() {
        let mecab = Mecab::new(PathBuf::from("mecab"));

        let token_list = mecab
            .tokenize(sentence().as_ref())
            .expect("Failed to tokenize sentence");

        assert_eq!(token_list, expected_token_list());
    }
}
